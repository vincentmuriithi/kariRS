#[doc = "Register `INTCTRL` reader"]
pub struct R(crate::R<SINGLE_INTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLE_INTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLE_INTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLE_INTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCTRL` writer"]
pub struct W(crate::W<SINGLE_INTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGLE_INTCTRL_SPEC>;
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
impl From<crate::W<SINGLE_INTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGLE_INTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVF` reader - Overflow Interrupt"]
pub type OVF_R = crate::BitReader<bool>;
#[doc = "Field `OVF` writer - Overflow Interrupt"]
pub type OVF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SINGLE_INTCTRL_SPEC, bool, O>;
#[doc = "Field `CMP0` reader - Compare 0 Interrupt"]
pub type CMP0_R = crate::BitReader<bool>;
#[doc = "Field `CMP0` writer - Compare 0 Interrupt"]
pub type CMP0_W<'a, const O: u8> = crate::BitWriter<'a, u8, SINGLE_INTCTRL_SPEC, bool, O>;
#[doc = "Field `CMP1` reader - Compare 1 Interrupt"]
pub type CMP1_R = crate::BitReader<bool>;
#[doc = "Field `CMP1` writer - Compare 1 Interrupt"]
pub type CMP1_W<'a, const O: u8> = crate::BitWriter<'a, u8, SINGLE_INTCTRL_SPEC, bool, O>;
#[doc = "Field `CMP2` reader - Compare 2 Interrupt"]
pub type CMP2_R = crate::BitReader<bool>;
#[doc = "Field `CMP2` writer - Compare 2 Interrupt"]
pub type CMP2_W<'a, const O: u8> = crate::BitWriter<'a, u8, SINGLE_INTCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Compare 0 Interrupt"]
    #[inline(always)]
    pub fn cmp0(&self) -> CMP0_R {
        CMP0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare 1 Interrupt"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare 2 Interrupt"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<0> {
        OVF_W::new(self)
    }
    #[doc = "Bit 4 - Compare 0 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0(&mut self) -> CMP0_W<4> {
        CMP0_W::new(self)
    }
    #[doc = "Bit 5 - Compare 1 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1(&mut self) -> CMP1_W<5> {
        CMP1_W::new(self)
    }
    #[doc = "Bit 6 - Compare 2 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2(&mut self) -> CMP2_W<6> {
        CMP2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [single_intctrl](index.html) module"]
pub struct SINGLE_INTCTRL_SPEC;
impl crate::RegisterSpec for SINGLE_INTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [single_intctrl::R](R) reader structure"]
impl crate::Readable for SINGLE_INTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [single_intctrl::W](W) writer structure"]
impl crate::Writable for SINGLE_INTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCTRL to value 0"]
impl crate::Resettable for SINGLE_INTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
