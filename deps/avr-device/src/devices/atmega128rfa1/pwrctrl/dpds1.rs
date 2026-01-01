#[doc = "Register `DPDS1` reader"]
pub struct R(crate::R<DPDS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPDS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPDS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPDS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPDS1` writer"]
pub struct W(crate::W<DPDS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPDS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DPDS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPDS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGDRV` reader - Driver Strength Port G"]
pub type PGDRV_R = crate::FieldReader<u8, PGDRV_A>;
#[doc = "Driver Strength Port G\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PGDRV_A {
    #[doc = "0: 2 mA"]
    PAD_IO_2MA = 0,
    #[doc = "1: 4 mA"]
    PAD_IO_4MA = 1,
    #[doc = "2: 6 mA"]
    PAD_IO_6MA = 2,
    #[doc = "3: 8 mA"]
    PAD_IO_8MA = 3,
}
impl From<PGDRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PGDRV_A) -> Self {
        variant as _
    }
}
impl PGDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGDRV_A {
        match self.bits {
            0 => PGDRV_A::PAD_IO_2MA,
            1 => PGDRV_A::PAD_IO_4MA,
            2 => PGDRV_A::PAD_IO_6MA,
            3 => PGDRV_A::PAD_IO_8MA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAD_IO_2MA`"]
    #[inline(always)]
    pub fn is_pad_io_2ma(&self) -> bool {
        *self == PGDRV_A::PAD_IO_2MA
    }
    #[doc = "Checks if the value of the field is `PAD_IO_4MA`"]
    #[inline(always)]
    pub fn is_pad_io_4ma(&self) -> bool {
        *self == PGDRV_A::PAD_IO_4MA
    }
    #[doc = "Checks if the value of the field is `PAD_IO_6MA`"]
    #[inline(always)]
    pub fn is_pad_io_6ma(&self) -> bool {
        *self == PGDRV_A::PAD_IO_6MA
    }
    #[doc = "Checks if the value of the field is `PAD_IO_8MA`"]
    #[inline(always)]
    pub fn is_pad_io_8ma(&self) -> bool {
        *self == PGDRV_A::PAD_IO_8MA
    }
}
#[doc = "Field `PGDRV` writer - Driver Strength Port G"]
pub type PGDRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, DPDS1_SPEC, u8, PGDRV_A, 2, O>;
impl<'a, const O: u8> PGDRV_W<'a, O> {
    #[doc = "2 mA"]
    #[inline(always)]
    pub fn pad_io_2ma(self) -> &'a mut W {
        self.variant(PGDRV_A::PAD_IO_2MA)
    }
    #[doc = "4 mA"]
    #[inline(always)]
    pub fn pad_io_4ma(self) -> &'a mut W {
        self.variant(PGDRV_A::PAD_IO_4MA)
    }
    #[doc = "6 mA"]
    #[inline(always)]
    pub fn pad_io_6ma(self) -> &'a mut W {
        self.variant(PGDRV_A::PAD_IO_6MA)
    }
    #[doc = "8 mA"]
    #[inline(always)]
    pub fn pad_io_8ma(self) -> &'a mut W {
        self.variant(PGDRV_A::PAD_IO_8MA)
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, DPDS1_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:1 - Driver Strength Port G"]
    #[inline(always)]
    pub fn pgdrv(&self) -> PGDRV_R {
        PGDRV_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 2) & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Driver Strength Port G"]
    #[inline(always)]
    #[must_use]
    pub fn pgdrv(&mut self) -> PGDRV_W<0> {
        PGDRV_W::new(self)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<2> {
        RES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Driver Strength Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpds1](index.html) module"]
pub struct DPDS1_SPEC;
impl crate::RegisterSpec for DPDS1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpds1::R](R) reader structure"]
impl crate::Readable for DPDS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpds1::W](W) writer structure"]
impl crate::Writable for DPDS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPDS1 to value 0"]
impl crate::Resettable for DPDS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
