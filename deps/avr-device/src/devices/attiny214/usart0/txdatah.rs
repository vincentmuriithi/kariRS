#[doc = "Register `TXDATAH` reader"]
pub struct R(crate::R<TXDATAH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDATAH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDATAH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDATAH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXDATAH` writer"]
pub struct W(crate::W<TXDATAH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDATAH_SPEC>;
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
impl From<crate::W<TXDATAH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDATAH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA8` reader - Transmit Data Register (CHSIZE=9bit)"]
pub type DATA8_R = crate::BitReader<bool>;
#[doc = "Field `DATA8` writer - Transmit Data Register (CHSIZE=9bit)"]
pub type DATA8_W<'a, const O: u8> = crate::BitWriter<'a, u8, TXDATAH_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmit Data Register (CHSIZE=9bit)"]
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data Register (CHSIZE=9bit)"]
    #[inline(always)]
    #[must_use]
    pub fn data8(&mut self) -> DATA8_W<0> {
        DATA8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Data High Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdatah](index.html) module"]
pub struct TXDATAH_SPEC;
impl crate::RegisterSpec for TXDATAH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [txdatah::R](R) reader structure"]
impl crate::Readable for TXDATAH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdatah::W](W) writer structure"]
impl crate::Writable for TXDATAH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDATAH to value 0"]
impl crate::Resettable for TXDATAH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
