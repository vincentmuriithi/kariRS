#[doc = "Register `EECR` reader"]
pub struct R(crate::R<EECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EECR` writer"]
pub struct W(crate::W<EECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EECR_SPEC>;
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
impl From<crate::W<EECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EERE` reader - EEPROM Read Enable"]
pub type EERE_R = crate::BitReader<bool>;
#[doc = "Field `EERE` writer - EEPROM Read Enable"]
pub type EERE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EECR_SPEC, bool, O>;
#[doc = "Field `EEPE` reader - EEPROM Write Enable"]
pub type EEPE_R = crate::BitReader<bool>;
#[doc = "Field `EEPE` writer - EEPROM Write Enable"]
pub type EEPE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EECR_SPEC, bool, O>;
#[doc = "Field `EEMPE` reader - EEPROM Master Write Enable"]
pub type EEMPE_R = crate::BitReader<bool>;
#[doc = "Field `EEMPE` writer - EEPROM Master Write Enable"]
pub type EEMPE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EECR_SPEC, bool, O>;
#[doc = "Field `EERIE` reader - EEProm Ready Interrupt Enable"]
pub type EERIE_R = crate::BitReader<bool>;
#[doc = "Field `EERIE` writer - EEProm Ready Interrupt Enable"]
pub type EERIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EECR_SPEC, bool, O>;
#[doc = "Field `EEPM` reader - No Description."]
pub type EEPM_R = crate::FieldReader<u8, EEPM_A>;
#[doc = "No Description.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EEPM_A {
    #[doc = "0: Atomic (erase and write in one operation)"]
    ATOMIC = 0,
    #[doc = "1: Erase only"]
    ERASE = 1,
    #[doc = "2: Write only"]
    WRITE = 2,
}
impl From<EEPM_A> for u8 {
    #[inline(always)]
    fn from(variant: EEPM_A) -> Self {
        variant as _
    }
}
impl EEPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EEPM_A> {
        match self.bits {
            0 => Some(EEPM_A::ATOMIC),
            1 => Some(EEPM_A::ERASE),
            2 => Some(EEPM_A::WRITE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ATOMIC`"]
    #[inline(always)]
    pub fn is_atomic(&self) -> bool {
        *self == EEPM_A::ATOMIC
    }
    #[doc = "Checks if the value of the field is `ERASE`"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == EEPM_A::ERASE
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == EEPM_A::WRITE
    }
}
#[doc = "Field `EEPM` writer - No Description."]
pub type EEPM_W<'a, const O: u8> = crate::FieldWriter<'a, u8, EECR_SPEC, u8, EEPM_A, 2, O>;
impl<'a, const O: u8> EEPM_W<'a, O> {
    #[doc = "Atomic (erase and write in one operation)"]
    #[inline(always)]
    pub fn atomic(self) -> &'a mut W {
        self.variant(EEPM_A::ATOMIC)
    }
    #[doc = "Erase only"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(EEPM_A::ERASE)
    }
    #[doc = "Write only"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(EEPM_A::WRITE)
    }
}
impl R {
    #[doc = "Bit 0 - EEPROM Read Enable"]
    #[inline(always)]
    pub fn eere(&self) -> EERE_R {
        EERE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EEPROM Write Enable"]
    #[inline(always)]
    pub fn eepe(&self) -> EEPE_R {
        EEPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EEPROM Master Write Enable"]
    #[inline(always)]
    pub fn eempe(&self) -> EEMPE_R {
        EEMPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EEProm Ready Interrupt Enable"]
    #[inline(always)]
    pub fn eerie(&self) -> EERIE_R {
        EERIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - No Description."]
    #[inline(always)]
    pub fn eepm(&self) -> EEPM_R {
        EEPM_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Read Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eere(&mut self) -> EERE_W<0> {
        EERE_W::new(self)
    }
    #[doc = "Bit 1 - EEPROM Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eepe(&mut self) -> EEPE_W<1> {
        EEPE_W::new(self)
    }
    #[doc = "Bit 2 - EEPROM Master Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eempe(&mut self) -> EEMPE_W<2> {
        EEMPE_W::new(self)
    }
    #[doc = "Bit 3 - EEProm Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eerie(&mut self) -> EERIE_W<3> {
        EERIE_W::new(self)
    }
    #[doc = "Bits 4:5 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn eepm(&mut self) -> EEPM_W<4> {
        EEPM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eecr](index.html) module"]
pub struct EECR_SPEC;
impl crate::RegisterSpec for EECR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eecr::R](R) reader structure"]
impl crate::Readable for EECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eecr::W](W) writer structure"]
impl crate::Writable for EECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EECR to value 0"]
impl crate::Resettable for EECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
