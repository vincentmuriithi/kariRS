#[doc = "Register `TCCR3B` reader"]
pub struct R(crate::R<TCCR3B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR3B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR3B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR3B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR3B` writer"]
pub struct W(crate::W<TCCR3B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR3B_SPEC>;
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
impl From<crate::W<TCCR3B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR3B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS3` reader - Clock Select3 bits"]
pub type CS3_R = crate::FieldReader<u8, CS3_A>;
#[doc = "Clock Select3 bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS3_A {
    #[doc = "0: No Clock Source (Stopped)"]
    VAL_0X00 = 0,
    #[doc = "1: Running, No Prescaling"]
    VAL_0X01 = 1,
    #[doc = "2: Running, CLK/8"]
    VAL_0X02 = 2,
    #[doc = "3: Running, CLK/64"]
    VAL_0X03 = 3,
    #[doc = "4: Running, CLK/256"]
    VAL_0X04 = 4,
    #[doc = "5: Running, CLK/1024"]
    VAL_0X05 = 5,
    #[doc = "6: Running, ExtClk Tx Falling Edge"]
    VAL_0X06 = 6,
    #[doc = "7: Running, ExtClk Tx Rising Edge"]
    VAL_0X07 = 7,
}
impl From<CS3_A> for u8 {
    #[inline(always)]
    fn from(variant: CS3_A) -> Self {
        variant as _
    }
}
impl CS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS3_A {
        match self.bits {
            0 => CS3_A::VAL_0X00,
            1 => CS3_A::VAL_0X01,
            2 => CS3_A::VAL_0X02,
            3 => CS3_A::VAL_0X03,
            4 => CS3_A::VAL_0X04,
            5 => CS3_A::VAL_0X05,
            6 => CS3_A::VAL_0X06,
            7 => CS3_A::VAL_0X07,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VAL_0X00`"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == CS3_A::VAL_0X00
    }
    #[doc = "Checks if the value of the field is `VAL_0X01`"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == CS3_A::VAL_0X01
    }
    #[doc = "Checks if the value of the field is `VAL_0X02`"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == CS3_A::VAL_0X02
    }
    #[doc = "Checks if the value of the field is `VAL_0X03`"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == CS3_A::VAL_0X03
    }
    #[doc = "Checks if the value of the field is `VAL_0X04`"]
    #[inline(always)]
    pub fn is_val_0x04(&self) -> bool {
        *self == CS3_A::VAL_0X04
    }
    #[doc = "Checks if the value of the field is `VAL_0X05`"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == CS3_A::VAL_0X05
    }
    #[doc = "Checks if the value of the field is `VAL_0X06`"]
    #[inline(always)]
    pub fn is_val_0x06(&self) -> bool {
        *self == CS3_A::VAL_0X06
    }
    #[doc = "Checks if the value of the field is `VAL_0X07`"]
    #[inline(always)]
    pub fn is_val_0x07(&self) -> bool {
        *self == CS3_A::VAL_0X07
    }
}
#[doc = "Field `CS3` writer - Clock Select3 bits"]
pub type CS3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR3B_SPEC, u8, CS3_A, 3, O>;
impl<'a, const O: u8> CS3_W<'a, O> {
    #[doc = "No Clock Source (Stopped)"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut W {
        self.variant(CS3_A::VAL_0X00)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut W {
        self.variant(CS3_A::VAL_0X01)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut W {
        self.variant(CS3_A::VAL_0X02)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut W {
        self.variant(CS3_A::VAL_0X03)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn val_0x04(self) -> &'a mut W {
        self.variant(CS3_A::VAL_0X04)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut W {
        self.variant(CS3_A::VAL_0X05)
    }
    #[doc = "Running, ExtClk Tx Falling Edge"]
    #[inline(always)]
    pub fn val_0x06(self) -> &'a mut W {
        self.variant(CS3_A::VAL_0X06)
    }
    #[doc = "Running, ExtClk Tx Rising Edge"]
    #[inline(always)]
    pub fn val_0x07(self) -> &'a mut W {
        self.variant(CS3_A::VAL_0X07)
    }
}
#[doc = "Field `WGM3` reader - Waveform Generation Mode"]
pub type WGM3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WGM3` writer - Waveform Generation Mode"]
pub type WGM3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR3B_SPEC, u8, u8, 2, O>;
#[doc = "Field `ICES3` reader - Input Capture 3 Edge Select"]
pub type ICES3_R = crate::BitReader<bool>;
#[doc = "Field `ICES3` writer - Input Capture 3 Edge Select"]
pub type ICES3_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR3B_SPEC, bool, O>;
#[doc = "Field `ICNC3` reader - Input Capture 3 Noise Canceler"]
pub type ICNC3_R = crate::BitReader<bool>;
#[doc = "Field `ICNC3` writer - Input Capture 3 Noise Canceler"]
pub type ICNC3_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR3B_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Clock Select3 bits"]
    #[inline(always)]
    pub fn cs3(&self) -> CS3_R {
        CS3_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm3(&self) -> WGM3_R {
        WGM3_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 6 - Input Capture 3 Edge Select"]
    #[inline(always)]
    pub fn ices3(&self) -> ICES3_R {
        ICES3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input Capture 3 Noise Canceler"]
    #[inline(always)]
    pub fn icnc3(&self) -> ICNC3_R {
        ICNC3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select3 bits"]
    #[inline(always)]
    #[must_use]
    pub fn cs3(&mut self) -> CS3_W<0> {
        CS3_W::new(self)
    }
    #[doc = "Bits 3:4 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm3(&mut self) -> WGM3_W<3> {
        WGM3_W::new(self)
    }
    #[doc = "Bit 6 - Input Capture 3 Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn ices3(&mut self) -> ICES3_W<6> {
        ICES3_W::new(self)
    }
    #[doc = "Bit 7 - Input Capture 3 Noise Canceler"]
    #[inline(always)]
    #[must_use]
    pub fn icnc3(&mut self) -> ICNC3_W<7> {
        ICNC3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter3 Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr3b](index.html) module"]
pub struct TCCR3B_SPEC;
impl crate::RegisterSpec for TCCR3B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr3b::R](R) reader structure"]
impl crate::Readable for TCCR3B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr3b::W](W) writer structure"]
impl crate::Writable for TCCR3B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR3B to value 0"]
impl crate::Resettable for TCCR3B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
