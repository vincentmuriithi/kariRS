#[doc = "Register `FAULTCTRL` reader"]
pub struct R(crate::R<FAULTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAULTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAULTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAULTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FAULTCTRL` writer"]
pub struct W(crate::W<FAULTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FAULTCTRL_SPEC>;
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
impl From<crate::W<FAULTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FAULTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPA` reader - Compare A value"]
pub type CMPA_R = crate::BitReader<bool>;
#[doc = "Field `CMPA` writer - Compare A value"]
pub type CMPA_W<'a, const O: u8> = crate::BitWriter<'a, u8, FAULTCTRL_SPEC, bool, O>;
#[doc = "Field `CMPB` reader - Compare B value"]
pub type CMPB_R = crate::BitReader<bool>;
#[doc = "Field `CMPB` writer - Compare B value"]
pub type CMPB_W<'a, const O: u8> = crate::BitWriter<'a, u8, FAULTCTRL_SPEC, bool, O>;
#[doc = "Field `CMPC` reader - Compare C value"]
pub type CMPC_R = crate::BitReader<bool>;
#[doc = "Field `CMPC` writer - Compare C value"]
pub type CMPC_W<'a, const O: u8> = crate::BitWriter<'a, u8, FAULTCTRL_SPEC, bool, O>;
#[doc = "Field `CMPD` reader - Compare D vaule"]
pub type CMPD_R = crate::BitReader<bool>;
#[doc = "Field `CMPD` writer - Compare D vaule"]
pub type CMPD_W<'a, const O: u8> = crate::BitWriter<'a, u8, FAULTCTRL_SPEC, bool, O>;
#[doc = "Field `CMPAEN` reader - Compare A enable"]
pub type CMPAEN_R = crate::BitReader<bool>;
#[doc = "Field `CMPAEN` writer - Compare A enable"]
pub type CMPAEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, FAULTCTRL_SPEC, bool, O>;
#[doc = "Field `CMPBEN` reader - Compare B enable"]
pub type CMPBEN_R = crate::BitReader<bool>;
#[doc = "Field `CMPBEN` writer - Compare B enable"]
pub type CMPBEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, FAULTCTRL_SPEC, bool, O>;
#[doc = "Field `CMPCEN` reader - Compare C enable"]
pub type CMPCEN_R = crate::BitReader<bool>;
#[doc = "Field `CMPCEN` writer - Compare C enable"]
pub type CMPCEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, FAULTCTRL_SPEC, bool, O>;
#[doc = "Field `CMPDEN` reader - Compare D enable"]
pub type CMPDEN_R = crate::BitReader<bool>;
#[doc = "Field `CMPDEN` writer - Compare D enable"]
pub type CMPDEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, FAULTCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Compare A value"]
    #[inline(always)]
    pub fn cmpa(&self) -> CMPA_R {
        CMPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare B value"]
    #[inline(always)]
    pub fn cmpb(&self) -> CMPB_R {
        CMPB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare C value"]
    #[inline(always)]
    pub fn cmpc(&self) -> CMPC_R {
        CMPC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare D vaule"]
    #[inline(always)]
    pub fn cmpd(&self) -> CMPD_R {
        CMPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare A enable"]
    #[inline(always)]
    pub fn cmpaen(&self) -> CMPAEN_R {
        CMPAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare B enable"]
    #[inline(always)]
    pub fn cmpben(&self) -> CMPBEN_R {
        CMPBEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare C enable"]
    #[inline(always)]
    pub fn cmpcen(&self) -> CMPCEN_R {
        CMPCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare D enable"]
    #[inline(always)]
    pub fn cmpden(&self) -> CMPDEN_R {
        CMPDEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare A value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpa(&mut self) -> CMPA_W<0> {
        CMPA_W::new(self)
    }
    #[doc = "Bit 1 - Compare B value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpb(&mut self) -> CMPB_W<1> {
        CMPB_W::new(self)
    }
    #[doc = "Bit 2 - Compare C value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpc(&mut self) -> CMPC_W<2> {
        CMPC_W::new(self)
    }
    #[doc = "Bit 3 - Compare D vaule"]
    #[inline(always)]
    #[must_use]
    pub fn cmpd(&mut self) -> CMPD_W<3> {
        CMPD_W::new(self)
    }
    #[doc = "Bit 4 - Compare A enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpaen(&mut self) -> CMPAEN_W<4> {
        CMPAEN_W::new(self)
    }
    #[doc = "Bit 5 - Compare B enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpben(&mut self) -> CMPBEN_W<5> {
        CMPBEN_W::new(self)
    }
    #[doc = "Bit 6 - Compare C enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcen(&mut self) -> CMPCEN_W<6> {
        CMPCEN_W::new(self)
    }
    #[doc = "Bit 7 - Compare D enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpden(&mut self) -> CMPDEN_W<7> {
        CMPDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faultctrl](index.html) module"]
pub struct FAULTCTRL_SPEC;
impl crate::RegisterSpec for FAULTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [faultctrl::R](R) reader structure"]
impl crate::Readable for FAULTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [faultctrl::W](W) writer structure"]
impl crate::Writable for FAULTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FAULTCTRL to value 0"]
impl crate::Resettable for FAULTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
