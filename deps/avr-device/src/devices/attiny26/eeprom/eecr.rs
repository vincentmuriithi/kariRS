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
#[doc = "Field `EEWE` reader - EEPROM Write Enable"]
pub type EEWE_R = crate::BitReader<bool>;
#[doc = "Field `EEWE` writer - EEPROM Write Enable"]
pub type EEWE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EECR_SPEC, bool, O>;
#[doc = "Field `EEMWE` reader - EEPROM Master Write Enable"]
pub type EEMWE_R = crate::BitReader<bool>;
#[doc = "Field `EEMWE` writer - EEPROM Master Write Enable"]
pub type EEMWE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EECR_SPEC, bool, O>;
#[doc = "Field `EERIE` reader - EEProm Ready Interrupt Enable"]
pub type EERIE_R = crate::BitReader<bool>;
#[doc = "Field `EERIE` writer - EEProm Ready Interrupt Enable"]
pub type EERIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EECR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EEPROM Read Enable"]
    #[inline(always)]
    pub fn eere(&self) -> EERE_R {
        EERE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EEPROM Write Enable"]
    #[inline(always)]
    pub fn eewe(&self) -> EEWE_R {
        EEWE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EEPROM Master Write Enable"]
    #[inline(always)]
    pub fn eemwe(&self) -> EEMWE_R {
        EEMWE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EEProm Ready Interrupt Enable"]
    #[inline(always)]
    pub fn eerie(&self) -> EERIE_R {
        EERIE_R::new(((self.bits >> 3) & 1) != 0)
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
    pub fn eewe(&mut self) -> EEWE_W<1> {
        EEWE_W::new(self)
    }
    #[doc = "Bit 2 - EEPROM Master Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eemwe(&mut self) -> EEMWE_W<2> {
        EEMWE_W::new(self)
    }
    #[doc = "Bit 3 - EEProm Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eerie(&mut self) -> EERIE_W<3> {
        EERIE_W::new(self)
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
