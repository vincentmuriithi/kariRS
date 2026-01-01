#[doc = "Register `TCCR2B` reader"]
pub struct R(crate::R<TCCR2B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR2B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR2B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR2B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR2B` writer"]
pub struct W(crate::W<TCCR2B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR2B_SPEC>;
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
impl From<crate::W<TCCR2B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR2B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS2` reader - Clock Select bits"]
pub type CS2_R = crate::FieldReader<u8, CS2_A>;
#[doc = "Clock Select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS2_A {
    #[doc = "0: No Clock Source (Stopped)"]
    VAL_0X00 = 0,
    #[doc = "1: Running, No Prescaling"]
    VAL_0X01 = 1,
    #[doc = "2: Running, CLK/8"]
    VAL_0X02 = 2,
    #[doc = "3: Running, CLK/32"]
    VAL_0X03 = 3,
    #[doc = "4: Running, CLK/64"]
    VAL_0X04 = 4,
    #[doc = "5: Running, CLK/128"]
    VAL_0X05 = 5,
    #[doc = "6: Running, CLK/256"]
    VAL_0X06 = 6,
    #[doc = "7: Running, CLK/1024"]
    VAL_0X07 = 7,
}
impl From<CS2_A> for u8 {
    #[inline(always)]
    fn from(variant: CS2_A) -> Self {
        variant as _
    }
}
impl CS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS2_A {
        match self.bits {
            0 => CS2_A::VAL_0X00,
            1 => CS2_A::VAL_0X01,
            2 => CS2_A::VAL_0X02,
            3 => CS2_A::VAL_0X03,
            4 => CS2_A::VAL_0X04,
            5 => CS2_A::VAL_0X05,
            6 => CS2_A::VAL_0X06,
            7 => CS2_A::VAL_0X07,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VAL_0X00`"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == CS2_A::VAL_0X00
    }
    #[doc = "Checks if the value of the field is `VAL_0X01`"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == CS2_A::VAL_0X01
    }
    #[doc = "Checks if the value of the field is `VAL_0X02`"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == CS2_A::VAL_0X02
    }
    #[doc = "Checks if the value of the field is `VAL_0X03`"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == CS2_A::VAL_0X03
    }
    #[doc = "Checks if the value of the field is `VAL_0X04`"]
    #[inline(always)]
    pub fn is_val_0x04(&self) -> bool {
        *self == CS2_A::VAL_0X04
    }
    #[doc = "Checks if the value of the field is `VAL_0X05`"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == CS2_A::VAL_0X05
    }
    #[doc = "Checks if the value of the field is `VAL_0X06`"]
    #[inline(always)]
    pub fn is_val_0x06(&self) -> bool {
        *self == CS2_A::VAL_0X06
    }
    #[doc = "Checks if the value of the field is `VAL_0X07`"]
    #[inline(always)]
    pub fn is_val_0x07(&self) -> bool {
        *self == CS2_A::VAL_0X07
    }
}
#[doc = "Field `CS2` writer - Clock Select bits"]
pub type CS2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR2B_SPEC, u8, CS2_A, 3, O>;
impl<'a, const O: u8> CS2_W<'a, O> {
    #[doc = "No Clock Source (Stopped)"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut W {
        self.variant(CS2_A::VAL_0X00)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut W {
        self.variant(CS2_A::VAL_0X01)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut W {
        self.variant(CS2_A::VAL_0X02)
    }
    #[doc = "Running, CLK/32"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut W {
        self.variant(CS2_A::VAL_0X03)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn val_0x04(self) -> &'a mut W {
        self.variant(CS2_A::VAL_0X04)
    }
    #[doc = "Running, CLK/128"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut W {
        self.variant(CS2_A::VAL_0X05)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn val_0x06(self) -> &'a mut W {
        self.variant(CS2_A::VAL_0X06)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn val_0x07(self) -> &'a mut W {
        self.variant(CS2_A::VAL_0X07)
    }
}
#[doc = "Field `WGM22` reader - Waveform Generation Mode"]
pub type WGM22_R = crate::BitReader<bool>;
#[doc = "Field `WGM22` writer - Waveform Generation Mode"]
pub type WGM22_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR2B_SPEC, bool, O>;
#[doc = "Field `FOC2B` reader - Force Output Compare B"]
pub type FOC2B_R = crate::BitReader<bool>;
#[doc = "Field `FOC2B` writer - Force Output Compare B"]
pub type FOC2B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR2B_SPEC, bool, O>;
#[doc = "Field `FOC2A` reader - Force Output Compare A"]
pub type FOC2A_R = crate::BitReader<bool>;
#[doc = "Field `FOC2A` writer - Force Output Compare A"]
pub type FOC2A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR2B_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Clock Select bits"]
    #[inline(always)]
    pub fn cs2(&self) -> CS2_R {
        CS2_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm22(&self) -> WGM22_R {
        WGM22_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Output Compare B"]
    #[inline(always)]
    pub fn foc2b(&self) -> FOC2B_R {
        FOC2B_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Output Compare A"]
    #[inline(always)]
    pub fn foc2a(&self) -> FOC2A_R {
        FOC2A_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn cs2(&mut self) -> CS2_W<0> {
        CS2_W::new(self)
    }
    #[doc = "Bit 3 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm22(&mut self) -> WGM22_W<3> {
        WGM22_W::new(self)
    }
    #[doc = "Bit 6 - Force Output Compare B"]
    #[inline(always)]
    #[must_use]
    pub fn foc2b(&mut self) -> FOC2B_W<6> {
        FOC2B_W::new(self)
    }
    #[doc = "Bit 7 - Force Output Compare A"]
    #[inline(always)]
    #[must_use]
    pub fn foc2a(&mut self) -> FOC2A_W<7> {
        FOC2A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter2 Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr2b](index.html) module"]
pub struct TCCR2B_SPEC;
impl crate::RegisterSpec for TCCR2B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr2b::R](R) reader structure"]
impl crate::Readable for TCCR2B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr2b::W](W) writer structure"]
impl crate::Writable for TCCR2B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR2B to value 0"]
impl crate::Resettable for TCCR2B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
