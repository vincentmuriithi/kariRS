#[doc = "Register `OUTCLR` reader"]
pub struct R(crate::R<OUTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTCLR` writer"]
pub struct W(crate::W<OUTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTCLR_SPEC>;
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
impl From<crate::W<OUTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC0` reader - Pin C0"]
pub type PC0_R = crate::BitReader<bool>;
#[doc = "Field `PC0` writer - Pin C0"]
pub type PC0_W<'a, const O: u8> = crate::BitWriter<'a, u8, OUTCLR_SPEC, bool, O>;
#[doc = "Field `PC1` reader - Pin C1"]
pub type PC1_R = crate::BitReader<bool>;
#[doc = "Field `PC1` writer - Pin C1"]
pub type PC1_W<'a, const O: u8> = crate::BitWriter<'a, u8, OUTCLR_SPEC, bool, O>;
#[doc = "Field `PC2` reader - Pin C2"]
pub type PC2_R = crate::BitReader<bool>;
#[doc = "Field `PC2` writer - Pin C2"]
pub type PC2_W<'a, const O: u8> = crate::BitWriter<'a, u8, OUTCLR_SPEC, bool, O>;
#[doc = "Field `PC3` reader - Pin C3"]
pub type PC3_R = crate::BitReader<bool>;
#[doc = "Field `PC3` writer - Pin C3"]
pub type PC3_W<'a, const O: u8> = crate::BitWriter<'a, u8, OUTCLR_SPEC, bool, O>;
#[doc = "Field `PC4` reader - Pin C4"]
pub type PC4_R = crate::BitReader<bool>;
#[doc = "Field `PC4` writer - Pin C4"]
pub type PC4_W<'a, const O: u8> = crate::BitWriter<'a, u8, OUTCLR_SPEC, bool, O>;
#[doc = "Field `PC5` reader - Pin C5"]
pub type PC5_R = crate::BitReader<bool>;
#[doc = "Field `PC5` writer - Pin C5"]
pub type PC5_W<'a, const O: u8> = crate::BitWriter<'a, u8, OUTCLR_SPEC, bool, O>;
#[doc = "Field `PC6` reader - Pin C6"]
pub type PC6_R = crate::BitReader<bool>;
#[doc = "Field `PC6` writer - Pin C6"]
pub type PC6_W<'a, const O: u8> = crate::BitWriter<'a, u8, OUTCLR_SPEC, bool, O>;
#[doc = "Field `PC7` reader - Pin C7"]
pub type PC7_R = crate::BitReader<bool>;
#[doc = "Field `PC7` writer - Pin C7"]
pub type PC7_W<'a, const O: u8> = crate::BitWriter<'a, u8, OUTCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pin C0"]
    #[inline(always)]
    pub fn pc0(&self) -> PC0_R {
        PC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin C1"]
    #[inline(always)]
    pub fn pc1(&self) -> PC1_R {
        PC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin C2"]
    #[inline(always)]
    pub fn pc2(&self) -> PC2_R {
        PC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin C3"]
    #[inline(always)]
    pub fn pc3(&self) -> PC3_R {
        PC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin C4"]
    #[inline(always)]
    pub fn pc4(&self) -> PC4_R {
        PC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin C5"]
    #[inline(always)]
    pub fn pc5(&self) -> PC5_R {
        PC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin C6"]
    #[inline(always)]
    pub fn pc6(&self) -> PC6_R {
        PC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin C7"]
    #[inline(always)]
    pub fn pc7(&self) -> PC7_R {
        PC7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin C0"]
    #[inline(always)]
    #[must_use]
    pub fn pc0(&mut self) -> PC0_W<0> {
        PC0_W::new(self)
    }
    #[doc = "Bit 1 - Pin C1"]
    #[inline(always)]
    #[must_use]
    pub fn pc1(&mut self) -> PC1_W<1> {
        PC1_W::new(self)
    }
    #[doc = "Bit 2 - Pin C2"]
    #[inline(always)]
    #[must_use]
    pub fn pc2(&mut self) -> PC2_W<2> {
        PC2_W::new(self)
    }
    #[doc = "Bit 3 - Pin C3"]
    #[inline(always)]
    #[must_use]
    pub fn pc3(&mut self) -> PC3_W<3> {
        PC3_W::new(self)
    }
    #[doc = "Bit 4 - Pin C4"]
    #[inline(always)]
    #[must_use]
    pub fn pc4(&mut self) -> PC4_W<4> {
        PC4_W::new(self)
    }
    #[doc = "Bit 5 - Pin C5"]
    #[inline(always)]
    #[must_use]
    pub fn pc5(&mut self) -> PC5_W<5> {
        PC5_W::new(self)
    }
    #[doc = "Bit 6 - Pin C6"]
    #[inline(always)]
    #[must_use]
    pub fn pc6(&mut self) -> PC6_W<6> {
        PC6_W::new(self)
    }
    #[doc = "Bit 7 - Pin C7"]
    #[inline(always)]
    #[must_use]
    pub fn pc7(&mut self) -> PC7_W<7> {
        PC7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Value Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outclr](index.html) module"]
pub struct OUTCLR_SPEC;
impl crate::RegisterSpec for OUTCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [outclr::R](R) reader structure"]
impl crate::Readable for OUTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outclr::W](W) writer structure"]
impl crate::Writable for OUTCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTCLR to value 0"]
impl crate::Resettable for OUTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
