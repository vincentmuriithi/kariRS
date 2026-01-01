#[doc = "Register `DPDS0` reader"]
pub struct R(crate::R<DPDS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPDS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPDS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPDS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPDS0` writer"]
pub struct W(crate::W<DPDS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPDS0_SPEC>;
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
impl From<crate::W<DPDS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPDS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBDRV` reader - Driver Strength Port B"]
pub type PBDRV_R = crate::FieldReader<u8, PBDRV_A>;
#[doc = "Driver Strength Port B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PBDRV_A {
    #[doc = "0: 2 mA"]
    PAD_IO_2MA = 0,
    #[doc = "1: 4 mA"]
    PAD_IO_4MA = 1,
    #[doc = "2: 6 mA"]
    PAD_IO_6MA = 2,
    #[doc = "3: 8 mA"]
    PAD_IO_8MA = 3,
}
impl From<PBDRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PBDRV_A) -> Self {
        variant as _
    }
}
impl PBDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBDRV_A {
        match self.bits {
            0 => PBDRV_A::PAD_IO_2MA,
            1 => PBDRV_A::PAD_IO_4MA,
            2 => PBDRV_A::PAD_IO_6MA,
            3 => PBDRV_A::PAD_IO_8MA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAD_IO_2MA`"]
    #[inline(always)]
    pub fn is_pad_io_2ma(&self) -> bool {
        *self == PBDRV_A::PAD_IO_2MA
    }
    #[doc = "Checks if the value of the field is `PAD_IO_4MA`"]
    #[inline(always)]
    pub fn is_pad_io_4ma(&self) -> bool {
        *self == PBDRV_A::PAD_IO_4MA
    }
    #[doc = "Checks if the value of the field is `PAD_IO_6MA`"]
    #[inline(always)]
    pub fn is_pad_io_6ma(&self) -> bool {
        *self == PBDRV_A::PAD_IO_6MA
    }
    #[doc = "Checks if the value of the field is `PAD_IO_8MA`"]
    #[inline(always)]
    pub fn is_pad_io_8ma(&self) -> bool {
        *self == PBDRV_A::PAD_IO_8MA
    }
}
#[doc = "Field `PBDRV` writer - Driver Strength Port B"]
pub type PBDRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, DPDS0_SPEC, u8, PBDRV_A, 2, O>;
impl<'a, const O: u8> PBDRV_W<'a, O> {
    #[doc = "2 mA"]
    #[inline(always)]
    pub fn pad_io_2ma(self) -> &'a mut W {
        self.variant(PBDRV_A::PAD_IO_2MA)
    }
    #[doc = "4 mA"]
    #[inline(always)]
    pub fn pad_io_4ma(self) -> &'a mut W {
        self.variant(PBDRV_A::PAD_IO_4MA)
    }
    #[doc = "6 mA"]
    #[inline(always)]
    pub fn pad_io_6ma(self) -> &'a mut W {
        self.variant(PBDRV_A::PAD_IO_6MA)
    }
    #[doc = "8 mA"]
    #[inline(always)]
    pub fn pad_io_8ma(self) -> &'a mut W {
        self.variant(PBDRV_A::PAD_IO_8MA)
    }
}
#[doc = "Field `PDDRV` reader - Driver Strength Port D"]
pub type PDDRV_R = crate::FieldReader<u8, PDDRV_A>;
#[doc = "Driver Strength Port D\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PDDRV_A {
    #[doc = "0: 2 mA"]
    PAD_IO_2MA = 0,
    #[doc = "1: 4 mA"]
    PAD_IO_4MA = 1,
    #[doc = "2: 6 mA"]
    PAD_IO_6MA = 2,
    #[doc = "3: 8 mA"]
    PAD_IO_8MA = 3,
}
impl From<PDDRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PDDRV_A) -> Self {
        variant as _
    }
}
impl PDDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDDRV_A {
        match self.bits {
            0 => PDDRV_A::PAD_IO_2MA,
            1 => PDDRV_A::PAD_IO_4MA,
            2 => PDDRV_A::PAD_IO_6MA,
            3 => PDDRV_A::PAD_IO_8MA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAD_IO_2MA`"]
    #[inline(always)]
    pub fn is_pad_io_2ma(&self) -> bool {
        *self == PDDRV_A::PAD_IO_2MA
    }
    #[doc = "Checks if the value of the field is `PAD_IO_4MA`"]
    #[inline(always)]
    pub fn is_pad_io_4ma(&self) -> bool {
        *self == PDDRV_A::PAD_IO_4MA
    }
    #[doc = "Checks if the value of the field is `PAD_IO_6MA`"]
    #[inline(always)]
    pub fn is_pad_io_6ma(&self) -> bool {
        *self == PDDRV_A::PAD_IO_6MA
    }
    #[doc = "Checks if the value of the field is `PAD_IO_8MA`"]
    #[inline(always)]
    pub fn is_pad_io_8ma(&self) -> bool {
        *self == PDDRV_A::PAD_IO_8MA
    }
}
#[doc = "Field `PDDRV` writer - Driver Strength Port D"]
pub type PDDRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, DPDS0_SPEC, u8, PDDRV_A, 2, O>;
impl<'a, const O: u8> PDDRV_W<'a, O> {
    #[doc = "2 mA"]
    #[inline(always)]
    pub fn pad_io_2ma(self) -> &'a mut W {
        self.variant(PDDRV_A::PAD_IO_2MA)
    }
    #[doc = "4 mA"]
    #[inline(always)]
    pub fn pad_io_4ma(self) -> &'a mut W {
        self.variant(PDDRV_A::PAD_IO_4MA)
    }
    #[doc = "6 mA"]
    #[inline(always)]
    pub fn pad_io_6ma(self) -> &'a mut W {
        self.variant(PDDRV_A::PAD_IO_6MA)
    }
    #[doc = "8 mA"]
    #[inline(always)]
    pub fn pad_io_8ma(self) -> &'a mut W {
        self.variant(PDDRV_A::PAD_IO_8MA)
    }
}
#[doc = "Field `PEDRV` reader - Driver Strength Port E"]
pub type PEDRV_R = crate::FieldReader<u8, PEDRV_A>;
#[doc = "Driver Strength Port E\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PEDRV_A {
    #[doc = "0: 2 mA"]
    PAD_IO_2MA = 0,
    #[doc = "1: 4 mA"]
    PAD_IO_4MA = 1,
    #[doc = "2: 6 mA"]
    PAD_IO_6MA = 2,
    #[doc = "3: 8 mA"]
    PAD_IO_8MA = 3,
}
impl From<PEDRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PEDRV_A) -> Self {
        variant as _
    }
}
impl PEDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEDRV_A {
        match self.bits {
            0 => PEDRV_A::PAD_IO_2MA,
            1 => PEDRV_A::PAD_IO_4MA,
            2 => PEDRV_A::PAD_IO_6MA,
            3 => PEDRV_A::PAD_IO_8MA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAD_IO_2MA`"]
    #[inline(always)]
    pub fn is_pad_io_2ma(&self) -> bool {
        *self == PEDRV_A::PAD_IO_2MA
    }
    #[doc = "Checks if the value of the field is `PAD_IO_4MA`"]
    #[inline(always)]
    pub fn is_pad_io_4ma(&self) -> bool {
        *self == PEDRV_A::PAD_IO_4MA
    }
    #[doc = "Checks if the value of the field is `PAD_IO_6MA`"]
    #[inline(always)]
    pub fn is_pad_io_6ma(&self) -> bool {
        *self == PEDRV_A::PAD_IO_6MA
    }
    #[doc = "Checks if the value of the field is `PAD_IO_8MA`"]
    #[inline(always)]
    pub fn is_pad_io_8ma(&self) -> bool {
        *self == PEDRV_A::PAD_IO_8MA
    }
}
#[doc = "Field `PEDRV` writer - Driver Strength Port E"]
pub type PEDRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, DPDS0_SPEC, u8, PEDRV_A, 2, O>;
impl<'a, const O: u8> PEDRV_W<'a, O> {
    #[doc = "2 mA"]
    #[inline(always)]
    pub fn pad_io_2ma(self) -> &'a mut W {
        self.variant(PEDRV_A::PAD_IO_2MA)
    }
    #[doc = "4 mA"]
    #[inline(always)]
    pub fn pad_io_4ma(self) -> &'a mut W {
        self.variant(PEDRV_A::PAD_IO_4MA)
    }
    #[doc = "6 mA"]
    #[inline(always)]
    pub fn pad_io_6ma(self) -> &'a mut W {
        self.variant(PEDRV_A::PAD_IO_6MA)
    }
    #[doc = "8 mA"]
    #[inline(always)]
    pub fn pad_io_8ma(self) -> &'a mut W {
        self.variant(PEDRV_A::PAD_IO_8MA)
    }
}
#[doc = "Field `PFDRV` reader - Driver Strength Port F"]
pub type PFDRV_R = crate::FieldReader<u8, PFDRV_A>;
#[doc = "Driver Strength Port F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PFDRV_A {
    #[doc = "0: 2 mA"]
    PAD_IO_2MA = 0,
    #[doc = "1: 4 mA"]
    PAD_IO_4MA = 1,
    #[doc = "2: 6 mA"]
    PAD_IO_6MA = 2,
    #[doc = "3: 8 mA"]
    PAD_IO_8MA = 3,
}
impl From<PFDRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PFDRV_A) -> Self {
        variant as _
    }
}
impl PFDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFDRV_A {
        match self.bits {
            0 => PFDRV_A::PAD_IO_2MA,
            1 => PFDRV_A::PAD_IO_4MA,
            2 => PFDRV_A::PAD_IO_6MA,
            3 => PFDRV_A::PAD_IO_8MA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAD_IO_2MA`"]
    #[inline(always)]
    pub fn is_pad_io_2ma(&self) -> bool {
        *self == PFDRV_A::PAD_IO_2MA
    }
    #[doc = "Checks if the value of the field is `PAD_IO_4MA`"]
    #[inline(always)]
    pub fn is_pad_io_4ma(&self) -> bool {
        *self == PFDRV_A::PAD_IO_4MA
    }
    #[doc = "Checks if the value of the field is `PAD_IO_6MA`"]
    #[inline(always)]
    pub fn is_pad_io_6ma(&self) -> bool {
        *self == PFDRV_A::PAD_IO_6MA
    }
    #[doc = "Checks if the value of the field is `PAD_IO_8MA`"]
    #[inline(always)]
    pub fn is_pad_io_8ma(&self) -> bool {
        *self == PFDRV_A::PAD_IO_8MA
    }
}
#[doc = "Field `PFDRV` writer - Driver Strength Port F"]
pub type PFDRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, DPDS0_SPEC, u8, PFDRV_A, 2, O>;
impl<'a, const O: u8> PFDRV_W<'a, O> {
    #[doc = "2 mA"]
    #[inline(always)]
    pub fn pad_io_2ma(self) -> &'a mut W {
        self.variant(PFDRV_A::PAD_IO_2MA)
    }
    #[doc = "4 mA"]
    #[inline(always)]
    pub fn pad_io_4ma(self) -> &'a mut W {
        self.variant(PFDRV_A::PAD_IO_4MA)
    }
    #[doc = "6 mA"]
    #[inline(always)]
    pub fn pad_io_6ma(self) -> &'a mut W {
        self.variant(PFDRV_A::PAD_IO_6MA)
    }
    #[doc = "8 mA"]
    #[inline(always)]
    pub fn pad_io_8ma(self) -> &'a mut W {
        self.variant(PFDRV_A::PAD_IO_8MA)
    }
}
impl R {
    #[doc = "Bits 0:1 - Driver Strength Port B"]
    #[inline(always)]
    pub fn pbdrv(&self) -> PBDRV_R {
        PBDRV_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Driver Strength Port D"]
    #[inline(always)]
    pub fn pddrv(&self) -> PDDRV_R {
        PDDRV_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Driver Strength Port E"]
    #[inline(always)]
    pub fn pedrv(&self) -> PEDRV_R {
        PEDRV_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Driver Strength Port F"]
    #[inline(always)]
    pub fn pfdrv(&self) -> PFDRV_R {
        PFDRV_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Driver Strength Port B"]
    #[inline(always)]
    #[must_use]
    pub fn pbdrv(&mut self) -> PBDRV_W<0> {
        PBDRV_W::new(self)
    }
    #[doc = "Bits 2:3 - Driver Strength Port D"]
    #[inline(always)]
    #[must_use]
    pub fn pddrv(&mut self) -> PDDRV_W<2> {
        PDDRV_W::new(self)
    }
    #[doc = "Bits 4:5 - Driver Strength Port E"]
    #[inline(always)]
    #[must_use]
    pub fn pedrv(&mut self) -> PEDRV_W<4> {
        PEDRV_W::new(self)
    }
    #[doc = "Bits 6:7 - Driver Strength Port F"]
    #[inline(always)]
    #[must_use]
    pub fn pfdrv(&mut self) -> PFDRV_W<6> {
        PFDRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Driver Strength Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpds0](index.html) module"]
pub struct DPDS0_SPEC;
impl crate::RegisterSpec for DPDS0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpds0::R](R) reader structure"]
impl crate::Readable for DPDS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpds0::W](W) writer structure"]
impl crate::Writable for DPDS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPDS0 to value 0"]
impl crate::Resettable for DPDS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
