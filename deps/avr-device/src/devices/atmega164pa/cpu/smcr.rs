#[doc = "Register `SMCR` reader"]
pub struct R(crate::R<SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCR` writer"]
pub struct W(crate::W<SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCR_SPEC>;
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
impl From<crate::W<SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SE` reader - Sleep Enable"]
pub type SE_R = crate::BitReader<bool>;
#[doc = "Field `SE` writer - Sleep Enable"]
pub type SE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SMCR_SPEC, bool, O>;
#[doc = "Field `SM` reader - Sleep Mode Select bits"]
pub type SM_R = crate::FieldReader<u8, SM_A>;
#[doc = "Sleep Mode Select bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SM_A {
    #[doc = "0: Idle"]
    IDLE = 0,
    #[doc = "1: ADC Noise Reduction (If Available)"]
    ADC = 1,
    #[doc = "2: Power Down"]
    PDOWN = 2,
    #[doc = "3: Power Save"]
    PSAVE = 3,
    #[doc = "4: Reserved"]
    VAL_0X04 = 4,
    #[doc = "5: Reserved"]
    VAL_0X05 = 5,
    #[doc = "6: Standby"]
    STDBY = 6,
    #[doc = "7: Extended Standby"]
    ESTDBY = 7,
}
impl From<SM_A> for u8 {
    #[inline(always)]
    fn from(variant: SM_A) -> Self {
        variant as _
    }
}
impl SM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM_A {
        match self.bits {
            0 => SM_A::IDLE,
            1 => SM_A::ADC,
            2 => SM_A::PDOWN,
            3 => SM_A::PSAVE,
            4 => SM_A::VAL_0X04,
            5 => SM_A::VAL_0X05,
            6 => SM_A::STDBY,
            7 => SM_A::ESTDBY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SM_A::IDLE
    }
    #[doc = "Checks if the value of the field is `ADC`"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == SM_A::ADC
    }
    #[doc = "Checks if the value of the field is `PDOWN`"]
    #[inline(always)]
    pub fn is_pdown(&self) -> bool {
        *self == SM_A::PDOWN
    }
    #[doc = "Checks if the value of the field is `PSAVE`"]
    #[inline(always)]
    pub fn is_psave(&self) -> bool {
        *self == SM_A::PSAVE
    }
    #[doc = "Checks if the value of the field is `VAL_0X04`"]
    #[inline(always)]
    pub fn is_val_0x04(&self) -> bool {
        *self == SM_A::VAL_0X04
    }
    #[doc = "Checks if the value of the field is `VAL_0X05`"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == SM_A::VAL_0X05
    }
    #[doc = "Checks if the value of the field is `STDBY`"]
    #[inline(always)]
    pub fn is_stdby(&self) -> bool {
        *self == SM_A::STDBY
    }
    #[doc = "Checks if the value of the field is `ESTDBY`"]
    #[inline(always)]
    pub fn is_estdby(&self) -> bool {
        *self == SM_A::ESTDBY
    }
}
#[doc = "Field `SM` writer - Sleep Mode Select bits"]
pub type SM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SMCR_SPEC, u8, SM_A, 3, O>;
impl<'a, const O: u8> SM_W<'a, O> {
    #[doc = "Idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(SM_A::IDLE)
    }
    #[doc = "ADC Noise Reduction (If Available)"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut W {
        self.variant(SM_A::ADC)
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub fn pdown(self) -> &'a mut W {
        self.variant(SM_A::PDOWN)
    }
    #[doc = "Power Save"]
    #[inline(always)]
    pub fn psave(self) -> &'a mut W {
        self.variant(SM_A::PSAVE)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn val_0x04(self) -> &'a mut W {
        self.variant(SM_A::VAL_0X04)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut W {
        self.variant(SM_A::VAL_0X05)
    }
    #[doc = "Standby"]
    #[inline(always)]
    pub fn stdby(self) -> &'a mut W {
        self.variant(SM_A::STDBY)
    }
    #[doc = "Extended Standby"]
    #[inline(always)]
    pub fn estdby(self) -> &'a mut W {
        self.variant(SM_A::ESTDBY)
    }
}
impl R {
    #[doc = "Bit 0 - Sleep Enable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Sleep Mode Select bits"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new((self.bits >> 1) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - Sleep Enable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<0> {
        SE_W::new(self)
    }
    #[doc = "Bits 1:3 - Sleep Mode Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SM_W<1> {
        SM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcr](index.html) module"]
pub struct SMCR_SPEC;
impl crate::RegisterSpec for SMCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [smcr::R](R) reader structure"]
impl crate::Readable for SMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smcr::W](W) writer structure"]
impl crate::Writable for SMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
