#[doc = "Register `TWAR` reader"]
pub struct R(crate::R<TWAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWAR` writer"]
pub struct W(crate::W<TWAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWAR_SPEC>;
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
impl From<crate::W<TWAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWGCE` reader - TWI General Call Recognition Enable Bit"]
pub type TWGCE_R = crate::BitReader<bool>;
#[doc = "Field `TWGCE` writer - TWI General Call Recognition Enable Bit"]
pub type TWGCE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TWAR_SPEC, bool, O>;
#[doc = "Field `TWA` reader - TWI (Slave) Address"]
pub type TWA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWA` writer - TWI (Slave) Address"]
pub type TWA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TWAR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - TWI General Call Recognition Enable Bit"]
    #[inline(always)]
    pub fn twgce(&self) -> TWGCE_R {
        TWGCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - TWI (Slave) Address"]
    #[inline(always)]
    pub fn twa(&self) -> TWA_R {
        TWA_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - TWI General Call Recognition Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn twgce(&mut self) -> TWGCE_W<0> {
        TWGCE_W::new(self)
    }
    #[doc = "Bits 1:7 - TWI (Slave) Address"]
    #[inline(always)]
    #[must_use]
    pub fn twa(&mut self) -> TWA_W<1> {
        TWA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI (Slave) Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twar](index.html) module"]
pub struct TWAR_SPEC;
impl crate::RegisterSpec for TWAR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [twar::R](R) reader structure"]
impl crate::Readable for TWAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twar::W](W) writer structure"]
impl crate::Writable for TWAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWAR to value 0"]
impl crate::Resettable for TWAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
