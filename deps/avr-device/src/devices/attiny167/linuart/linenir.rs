#[doc = "Register `LINENIR` reader"]
pub struct R(crate::R<LINENIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINENIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINENIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINENIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINENIR` writer"]
pub struct W(crate::W<LINENIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINENIR_SPEC>;
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
impl From<crate::W<LINENIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINENIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LENRXOK` reader - Enable Receive Performed Interrupt"]
pub type LENRXOK_R = crate::BitReader<bool>;
#[doc = "Field `LENRXOK` writer - Enable Receive Performed Interrupt"]
pub type LENRXOK_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINENIR_SPEC, bool, O>;
#[doc = "Field `LENTXOK` reader - Enable Transmit Performed Interrupt"]
pub type LENTXOK_R = crate::BitReader<bool>;
#[doc = "Field `LENTXOK` writer - Enable Transmit Performed Interrupt"]
pub type LENTXOK_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINENIR_SPEC, bool, O>;
#[doc = "Field `LENIDOK` reader - Enable Identifier Interrupt"]
pub type LENIDOK_R = crate::BitReader<bool>;
#[doc = "Field `LENIDOK` writer - Enable Identifier Interrupt"]
pub type LENIDOK_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINENIR_SPEC, bool, O>;
#[doc = "Field `LENERR` reader - Enable Error Interrupt"]
pub type LENERR_R = crate::BitReader<bool>;
#[doc = "Field `LENERR` writer - Enable Error Interrupt"]
pub type LENERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINENIR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable Receive Performed Interrupt"]
    #[inline(always)]
    pub fn lenrxok(&self) -> LENRXOK_R {
        LENRXOK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Transmit Performed Interrupt"]
    #[inline(always)]
    pub fn lentxok(&self) -> LENTXOK_R {
        LENTXOK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Identifier Interrupt"]
    #[inline(always)]
    pub fn lenidok(&self) -> LENIDOK_R {
        LENIDOK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Error Interrupt"]
    #[inline(always)]
    pub fn lenerr(&self) -> LENERR_R {
        LENERR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Receive Performed Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lenrxok(&mut self) -> LENRXOK_W<0> {
        LENRXOK_W::new(self)
    }
    #[doc = "Bit 1 - Enable Transmit Performed Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lentxok(&mut self) -> LENTXOK_W<1> {
        LENTXOK_W::new(self)
    }
    #[doc = "Bit 2 - Enable Identifier Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lenidok(&mut self) -> LENIDOK_W<2> {
        LENIDOK_W::new(self)
    }
    #[doc = "Bit 3 - Enable Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lenerr(&mut self) -> LENERR_W<3> {
        LENERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LIN Enable Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linenir](index.html) module"]
pub struct LINENIR_SPEC;
impl crate::RegisterSpec for LINENIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [linenir::R](R) reader structure"]
impl crate::Readable for LINENIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [linenir::W](W) writer structure"]
impl crate::Writable for LINENIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINENIR to value 0"]
impl crate::Resettable for LINENIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
