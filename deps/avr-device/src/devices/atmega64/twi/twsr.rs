#[doc = "Register `TWSR` reader"]
pub struct R(crate::R<TWSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWSR` writer"]
pub struct W(crate::W<TWSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWSR_SPEC>;
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
impl From<crate::W<TWSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWPS` reader - TWI Prescaler"]
pub type TWPS_R = crate::FieldReader<u8, TWPS_A>;
#[doc = "TWI Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TWPS_A {
    #[doc = "0: 1"]
    VAL_0X00 = 0,
    #[doc = "1: 4"]
    VAL_0X01 = 1,
    #[doc = "2: 16"]
    VAL_0X02 = 2,
    #[doc = "3: 64"]
    VAL_0X03 = 3,
}
impl From<TWPS_A> for u8 {
    #[inline(always)]
    fn from(variant: TWPS_A) -> Self {
        variant as _
    }
}
impl TWPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWPS_A {
        match self.bits {
            0 => TWPS_A::VAL_0X00,
            1 => TWPS_A::VAL_0X01,
            2 => TWPS_A::VAL_0X02,
            3 => TWPS_A::VAL_0X03,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VAL_0X00`"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == TWPS_A::VAL_0X00
    }
    #[doc = "Checks if the value of the field is `VAL_0X01`"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == TWPS_A::VAL_0X01
    }
    #[doc = "Checks if the value of the field is `VAL_0X02`"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == TWPS_A::VAL_0X02
    }
    #[doc = "Checks if the value of the field is `VAL_0X03`"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == TWPS_A::VAL_0X03
    }
}
#[doc = "Field `TWPS` writer - TWI Prescaler"]
pub type TWPS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TWSR_SPEC, u8, TWPS_A, 2, O>;
impl<'a, const O: u8> TWPS_W<'a, O> {
    #[doc = "1"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut W {
        self.variant(TWPS_A::VAL_0X00)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut W {
        self.variant(TWPS_A::VAL_0X01)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut W {
        self.variant(TWPS_A::VAL_0X02)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut W {
        self.variant(TWPS_A::VAL_0X03)
    }
}
#[doc = "Field `TWS` reader - TWI Status"]
pub type TWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWS` writer - TWI Status"]
pub type TWS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TWSR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:1 - TWI Prescaler"]
    #[inline(always)]
    pub fn twps(&self) -> TWPS_R {
        TWPS_R::new(self.bits & 3)
    }
    #[doc = "Bits 3:7 - TWI Status"]
    #[inline(always)]
    pub fn tws(&self) -> TWS_R {
        TWS_R::new((self.bits >> 3) & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:1 - TWI Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn twps(&mut self) -> TWPS_W<0> {
        TWPS_W::new(self)
    }
    #[doc = "Bits 3:7 - TWI Status"]
    #[inline(always)]
    #[must_use]
    pub fn tws(&mut self) -> TWS_W<3> {
        TWS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twsr](index.html) module"]
pub struct TWSR_SPEC;
impl crate::RegisterSpec for TWSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [twsr::R](R) reader structure"]
impl crate::Readable for TWSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twsr::W](W) writer structure"]
impl crate::Writable for TWSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWSR to value 0"]
impl crate::Resettable for TWSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
