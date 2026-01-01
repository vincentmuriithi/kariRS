#[doc = "Register `DDRE` reader"]
pub struct R(crate::R<DDRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRE` writer"]
pub struct W(crate::W<DDRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRE_SPEC>;
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
impl From<crate::W<DDRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PE0` reader - Pin E0"]
pub type PE0_R = crate::BitReader<bool>;
#[doc = "Field `PE0` writer - Pin E0"]
pub type PE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRE_SPEC, bool, O>;
#[doc = "Field `PE1` reader - Pin E1"]
pub type PE1_R = crate::BitReader<bool>;
#[doc = "Field `PE1` writer - Pin E1"]
pub type PE1_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRE_SPEC, bool, O>;
#[doc = "Field `PE2` reader - Pin E2"]
pub type PE2_R = crate::BitReader<bool>;
#[doc = "Field `PE2` writer - Pin E2"]
pub type PE2_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRE_SPEC, bool, O>;
#[doc = "Field `PE3` reader - Pin E3"]
pub type PE3_R = crate::BitReader<bool>;
#[doc = "Field `PE3` writer - Pin E3"]
pub type PE3_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRE_SPEC, bool, O>;
#[doc = "Field `PE4` reader - Pin E4"]
pub type PE4_R = crate::BitReader<bool>;
#[doc = "Field `PE4` writer - Pin E4"]
pub type PE4_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRE_SPEC, bool, O>;
#[doc = "Field `PE5` reader - Pin E5"]
pub type PE5_R = crate::BitReader<bool>;
#[doc = "Field `PE5` writer - Pin E5"]
pub type PE5_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRE_SPEC, bool, O>;
#[doc = "Field `PE6` reader - Pin E6"]
pub type PE6_R = crate::BitReader<bool>;
#[doc = "Field `PE6` writer - Pin E6"]
pub type PE6_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRE_SPEC, bool, O>;
#[doc = "Field `PE7` reader - Pin E7"]
pub type PE7_R = crate::BitReader<bool>;
#[doc = "Field `PE7` writer - Pin E7"]
pub type PE7_W<'a, const O: u8> = crate::BitWriter<'a, u8, DDRE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pin E0"]
    #[inline(always)]
    pub fn pe0(&self) -> PE0_R {
        PE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin E1"]
    #[inline(always)]
    pub fn pe1(&self) -> PE1_R {
        PE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin E2"]
    #[inline(always)]
    pub fn pe2(&self) -> PE2_R {
        PE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin E3"]
    #[inline(always)]
    pub fn pe3(&self) -> PE3_R {
        PE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin E4"]
    #[inline(always)]
    pub fn pe4(&self) -> PE4_R {
        PE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin E5"]
    #[inline(always)]
    pub fn pe5(&self) -> PE5_R {
        PE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin E6"]
    #[inline(always)]
    pub fn pe6(&self) -> PE6_R {
        PE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin E7"]
    #[inline(always)]
    pub fn pe7(&self) -> PE7_R {
        PE7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin E0"]
    #[inline(always)]
    #[must_use]
    pub fn pe0(&mut self) -> PE0_W<0> {
        PE0_W::new(self)
    }
    #[doc = "Bit 1 - Pin E1"]
    #[inline(always)]
    #[must_use]
    pub fn pe1(&mut self) -> PE1_W<1> {
        PE1_W::new(self)
    }
    #[doc = "Bit 2 - Pin E2"]
    #[inline(always)]
    #[must_use]
    pub fn pe2(&mut self) -> PE2_W<2> {
        PE2_W::new(self)
    }
    #[doc = "Bit 3 - Pin E3"]
    #[inline(always)]
    #[must_use]
    pub fn pe3(&mut self) -> PE3_W<3> {
        PE3_W::new(self)
    }
    #[doc = "Bit 4 - Pin E4"]
    #[inline(always)]
    #[must_use]
    pub fn pe4(&mut self) -> PE4_W<4> {
        PE4_W::new(self)
    }
    #[doc = "Bit 5 - Pin E5"]
    #[inline(always)]
    #[must_use]
    pub fn pe5(&mut self) -> PE5_W<5> {
        PE5_W::new(self)
    }
    #[doc = "Bit 6 - Pin E6"]
    #[inline(always)]
    #[must_use]
    pub fn pe6(&mut self) -> PE6_W<6> {
        PE6_W::new(self)
    }
    #[doc = "Bit 7 - Pin E7"]
    #[inline(always)]
    #[must_use]
    pub fn pe7(&mut self) -> PE7_W<7> {
        PE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Direction Register, Port E\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddre](index.html) module"]
pub struct DDRE_SPEC;
impl crate::RegisterSpec for DDRE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ddre::R](R) reader structure"]
impl crate::Readable for DDRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddre::W](W) writer structure"]
impl crate::Writable for DDRE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRE to value 0"]
impl crate::Resettable for DDRE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
