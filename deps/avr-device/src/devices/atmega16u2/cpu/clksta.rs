#[doc = "Register `CLKSTA` reader"]
pub struct R(crate::R<CLKSTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKSTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKSTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKSTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKSTA` writer"]
pub struct W(crate::W<CLKSTA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKSTA_SPEC>;
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
impl From<crate::W<CLKSTA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKSTA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTON` reader - No Description."]
pub type EXTON_R = crate::BitReader<bool>;
#[doc = "Field `EXTON` writer - No Description."]
pub type EXTON_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLKSTA_SPEC, bool, O>;
#[doc = "Field `RCON` reader - No Description."]
pub type RCON_R = crate::BitReader<bool>;
#[doc = "Field `RCON` writer - No Description."]
pub type RCON_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLKSTA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn exton(&self) -> EXTON_R {
        EXTON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    pub fn rcon(&self) -> RCON_R {
        RCON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn exton(&mut self) -> EXTON_W<0> {
        EXTON_W::new(self)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rcon(&mut self) -> RCON_W<1> {
        RCON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clksta](index.html) module"]
pub struct CLKSTA_SPEC;
impl crate::RegisterSpec for CLKSTA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clksta::R](R) reader structure"]
impl crate::Readable for CLKSTA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clksta::W](W) writer structure"]
impl crate::Writable for CLKSTA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKSTA to value 0"]
impl crate::Resettable for CLKSTA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
