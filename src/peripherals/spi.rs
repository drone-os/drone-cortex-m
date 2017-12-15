//! Serial peripheral interface.

#[allow(unused_imports)]
use core::marker::PhantomData;
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
use core::ptr::{read_volatile, write_volatile};
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
use reg::{spi1, spi2, spi3};
use reg::prelude::*;
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
use thread::interrupts::{IrqSpi1, IrqSpi2, IrqSpi3};

/// Generic SPI.
#[allow(missing_docs)]
pub trait Spi<T: Thread, I: ThreadBinding<T>>: Sized {
  /// Concrete SPI input items.
  type Input;

  /// Concrete SPI output items.
  type Output;

  type Cr1: Reg<Fbt>;
  type Cr2: Reg<Fbt>;
  type Crcpr: Reg<Sbt>;
  type Dr: Reg<Sbt>;
  type Rxcrcr: Reg<Sbt>;
  type Sr: Reg<Sbt>;
  type Txcrcr: Reg<Sbt>;

  /// Composes a new `Spi` from pieces.
  fn compose(input: Self::Input) -> Self;

  /// Decomposes the `Spi` into pieces.
  fn decompose(self) -> Self::Output;

  fn irq(&self) -> I;
  fn cr1(&self) -> &Self::Cr1;
  fn cr2(&self) -> &Self::Cr2;
  fn crcpr(&self) -> &Self::Crcpr;
  fn dr(&self) -> &Self::Dr;
  fn rxcrcr(&self) -> &Self::Rxcrcr;
  fn sr(&self) -> &Self::Sr;
  fn txcrcr(&self) -> &Self::Txcrcr;

  /// Writes `u8` value to the data register.
  fn send_byte(&self, value: u8);

  /// Writes `u16` value to the data register.
  fn send_hword(&self, value: u16);

  /// Reads `u8` value from the data register.
  fn recv_byte(&self) -> u8;

  /// Reads `u16` value from the data register.
  fn recv_hword(&self) -> u16;

  /// Moves `self` into `f` while `SPE` is cleared, and then sets `SPE`.
  fn spe_after<F, R>(self, cr1_val: <Self::Cr1 as Reg<Fbt>>::Val, f: F) -> R
  where
    F: FnOnce(Self) -> R;

  /// Moves `self` into `f` while `TXDMAEN` is cleared, and then sets `TXDMAEN`.
  fn txdmaen_after<F, R>(
    self,
    cr2_val: <Self::Cr2 as Reg<Fbt>>::Val,
    f: F,
  ) -> R
  where
    F: FnOnce(Self) -> R;
}

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
macro_rules! spi {
  (
    $doc:expr,
    $name:ident,
    $doc_items:expr,
    $name_items:ident,
    $irq_ty:ident,
    $spi:ident,
    $spi_cr1:ident,
    $spi_cr2:ident,
    $spi_crcpr:ident,
    $spi_dr:ident,
    $spi_rxcrcr:ident,
    $spi_sr:ident,
    $spi_txcrcr:ident,
  ) => {
    #[doc = $doc]
    pub struct $name<T: Thread, I: $irq_ty<T>> {
      _thread: PhantomData<&'static T>,
      irq: I,
      cr1: $spi::Cr1<Fbt>,
      cr2: $spi::Cr2<Fbt>,
      crcpr: $spi::Crcpr<Sbt>,
      dr: $spi::Dr<Sbt>,
      rxcrcr: $spi::Rxcrcr<Sbt>,
      sr: $spi::Sr<Sbt>,
      txcrcr: $spi::Txcrcr<Sbt>,
    }

    #[doc = $doc_items]
    #[allow(missing_docs)]
    pub struct $name_items<T: Thread, I: $irq_ty<T>, R: RegTag> {
      pub thread: PhantomData<&'static T>,
      pub $spi: I,
      pub $spi_cr1: $spi::Cr1<R>,
      pub $spi_cr2: $spi::Cr2<R>,
      pub $spi_crcpr: $spi::Crcpr<Sbt>,
      pub $spi_dr: $spi::Dr<Sbt>,
      pub $spi_rxcrcr: $spi::Rxcrcr<Sbt>,
      pub $spi_sr: $spi::Sr<Sbt>,
      pub $spi_txcrcr: $spi::Txcrcr<Sbt>,
    }

    /// Composes a new `Spi` from pieces.
    pub macro $name($threads:ident, $regs:ident) {
      $crate::peripherals::spi::Spi::compose(
        $crate::peripherals::spi::$name_items {
          thread: $crate::peripherals::rt::PhantomData,
          $spi: $threads.$spi,
          $spi_cr1: $regs.$spi_cr1,
          $spi_cr2: $regs.$spi_cr2,
          $spi_crcpr: $regs.$spi_crcpr,
          $spi_dr: $regs.$spi_dr,
          $spi_rxcrcr: $regs.$spi_rxcrcr,
          $spi_sr: $regs.$spi_sr,
          $spi_txcrcr: $regs.$spi_txcrcr,
        }
      )
    }

    impl<T: Thread, I: $irq_ty<T>> Spi<T, I> for $name<T, I> {
      type Input = $name_items<T, I, Sbt>;
      type Output = $name_items<T, I, Fbt>;
      type Cr1 = $spi::Cr1<Fbt>;
      type Cr2 = $spi::Cr2<Fbt>;
      type Crcpr = $spi::Crcpr<Sbt>;
      type Dr = $spi::Dr<Sbt>;
      type Rxcrcr = $spi::Rxcrcr<Sbt>;
      type Sr = $spi::Sr<Sbt>;
      type Txcrcr = $spi::Txcrcr<Sbt>;

      #[inline(always)]
      fn compose(input: Self::Input) -> Self {
        Self {
          _thread: PhantomData,
          irq: input.$spi,
          cr1: input.$spi_cr1.into(),
          cr2: input.$spi_cr2.into(),
          crcpr: input.$spi_crcpr,
          dr: input.$spi_dr,
          rxcrcr: input.$spi_rxcrcr,
          sr: input.$spi_sr,
          txcrcr: input.$spi_txcrcr,
        }
      }

      #[inline(always)]
      fn decompose(self) -> Self::Output {
        Self::Output {
          thread: PhantomData,
          $spi: self.irq,
          $spi_cr1: self.cr1,
          $spi_cr2: self.cr2,
          $spi_crcpr: self.crcpr,
          $spi_dr: self.dr,
          $spi_rxcrcr: self.rxcrcr,
          $spi_sr: self.sr,
          $spi_txcrcr: self.txcrcr,
        }
      }

      #[inline(always)]
      fn irq(&self) -> I {
        self.irq
      }

      #[inline(always)]
      fn cr1(&self) -> &Self::Cr1 {
        &self.cr1
      }

      #[inline(always)]
      fn cr2(&self) -> &Self::Cr2 {
        &self.cr2
      }

      #[inline(always)]
      fn crcpr(&self) -> &Self::Crcpr {
        &self.crcpr
      }

      #[inline(always)]
      fn dr(&self) -> &Self::Dr {
        &self.dr
      }

      #[inline(always)]
      fn rxcrcr(&self) -> &Self::Rxcrcr {
        &self.rxcrcr
      }

      #[inline(always)]
      fn sr(&self) -> &Self::Sr {
        &self.sr
      }

      #[inline(always)]
      fn txcrcr(&self) -> &Self::Txcrcr {
        &self.txcrcr
      }

      #[inline(always)]
      fn send_byte(&self, value: u8) {
        unsafe { write_volatile(self.dr.to_mut_ptr() as *mut _, value) };
      }

      #[inline(always)]
      fn send_hword(&self, value: u16) {
        unsafe { write_volatile(self.dr.to_mut_ptr() as *mut _, value) };
      }

      #[inline(always)]
      fn recv_byte(&self) -> u8 {
        unsafe { read_volatile(self.dr.to_ptr() as *mut _) }
      }

      #[inline(always)]
      fn recv_hword(&self) -> u16 {
        unsafe { read_volatile(self.dr.to_ptr() as *mut _) }
      }

      #[inline]
      fn spe_after<F, R>(
        mut self,
        mut cr1_val: <Self::Cr1 as Reg<Fbt>>::Val,
        f: F,
      ) -> R
      where
        F: FnOnce(Self) -> R,
      {
        let cr1 = self.cr1.fork();
        let cr1_spe = self.cr1.spe.fork();
        cr1_spe.clear(&mut cr1_val);
        cr1.store_val(cr1_val);
        let result = f(self);
        cr1_spe.set(&mut cr1_val);
        cr1.store_val(cr1_val);
        result
      }

      #[inline]
      fn txdmaen_after<F, R>(
        mut self,
        mut cr2_val: <Self::Cr2 as Reg<Fbt>>::Val,
        f: F,
      ) -> R
      where
        F: FnOnce(Self) -> R,
      {
        let cr2 = self.cr2.fork();
        let cr2_txdmaen = self.cr2.txdmaen.fork();
        cr2_txdmaen.clear(&mut cr2_val);
        cr2.store_val(cr2_val);
        let result = f(self);
        cr2_txdmaen.set(&mut cr2_val);
        cr2.store_val(cr2_val);
        result
      }
    }
  }
}

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
spi! {
  "SPI1.",
  Spi1,
  "SPI1 items.",
  Spi1Items,
  IrqSpi1,
  spi1,
  spi1_cr1,
  spi1_cr2,
  spi1_crcpr,
  spi1_dr,
  spi1_rxcrcr,
  spi1_sr,
  spi1_txcrcr,
}

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
spi! {
  "SPI2.",
  Spi2,
  "SPI2 items.",
  Spi2Items,
  IrqSpi2,
  spi2,
  spi2_cr1,
  spi2_cr2,
  spi2_crcpr,
  spi2_dr,
  spi2_rxcrcr,
  spi2_sr,
  spi2_txcrcr,
}

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
spi! {
  "SPI3.",
  Spi3,
  "SPI3 items.",
  Spi3Items,
  IrqSpi3,
  spi3,
  spi3_cr1,
  spi3_cr2,
  spi3_crcpr,
  spi3_dr,
  spi3_rxcrcr,
  spi3_sr,
  spi3_txcrcr,
}
