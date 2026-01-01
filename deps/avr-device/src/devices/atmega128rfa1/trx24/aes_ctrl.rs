#[doc = "Register `AES_CTRL` reader"]
pub struct R(crate::R<AES_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AES_CTRL` writer"]
pub struct W(crate::W<AES_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_CTRL_SPEC>;
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
impl From<crate::W<AES_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_IM` reader - AES Interrupt Enable"]
pub type AES_IM_R = crate::BitReader<bool>;
#[doc = "Field `AES_IM` writer - AES Interrupt Enable"]
pub type AES_IM_W<'a, const O: u8> = crate::BitWriter<'a, u8, AES_CTRL_SPEC, bool, O>;
#[doc = "Field `AES_DIR` reader - Set AES Operation Direction"]
pub type AES_DIR_R = crate::BitReader<AES_DIR_A>;
#[doc = "Set AES Operation Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AES_DIR_A {
    #[doc = "0: AES operation is encryption."]
    AES_DIR_ENC = 0,
    #[doc = "1: AES operation is decryption."]
    AES_DIR_DEC = 1,
}
impl From<AES_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: AES_DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl AES_DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AES_DIR_A {
        match self.bits {
            false => AES_DIR_A::AES_DIR_ENC,
            true => AES_DIR_A::AES_DIR_DEC,
        }
    }
    #[doc = "Checks if the value of the field is `AES_DIR_ENC`"]
    #[inline(always)]
    pub fn is_aes_dir_enc(&self) -> bool {
        *self == AES_DIR_A::AES_DIR_ENC
    }
    #[doc = "Checks if the value of the field is `AES_DIR_DEC`"]
    #[inline(always)]
    pub fn is_aes_dir_dec(&self) -> bool {
        *self == AES_DIR_A::AES_DIR_DEC
    }
}
#[doc = "Field `AES_DIR` writer - Set AES Operation Direction"]
pub type AES_DIR_W<'a, const O: u8> = crate::BitWriter<'a, u8, AES_CTRL_SPEC, AES_DIR_A, O>;
impl<'a, const O: u8> AES_DIR_W<'a, O> {
    #[doc = "AES operation is encryption."]
    #[inline(always)]
    pub fn aes_dir_enc(self) -> &'a mut W {
        self.variant(AES_DIR_A::AES_DIR_ENC)
    }
    #[doc = "AES operation is decryption."]
    #[inline(always)]
    pub fn aes_dir_dec(self) -> &'a mut W {
        self.variant(AES_DIR_A::AES_DIR_DEC)
    }
}
#[doc = "Field `AES_MODE` reader - Set AES Operation Mode"]
pub type AES_MODE_R = crate::BitReader<AES_MODE_A>;
#[doc = "Set AES Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AES_MODE_A {
    #[doc = "0: AES Mode is ECB (Electronic Code Book)."]
    AES_MODE_ECB = 0,
    #[doc = "1: AES Mode is CBC (Cipher Block Chaining)."]
    AES_MODE_CBC = 1,
}
impl From<AES_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: AES_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl AES_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AES_MODE_A {
        match self.bits {
            false => AES_MODE_A::AES_MODE_ECB,
            true => AES_MODE_A::AES_MODE_CBC,
        }
    }
    #[doc = "Checks if the value of the field is `AES_MODE_ECB`"]
    #[inline(always)]
    pub fn is_aes_mode_ecb(&self) -> bool {
        *self == AES_MODE_A::AES_MODE_ECB
    }
    #[doc = "Checks if the value of the field is `AES_MODE_CBC`"]
    #[inline(always)]
    pub fn is_aes_mode_cbc(&self) -> bool {
        *self == AES_MODE_A::AES_MODE_CBC
    }
}
#[doc = "Field `AES_MODE` writer - Set AES Operation Mode"]
pub type AES_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u8, AES_CTRL_SPEC, AES_MODE_A, O>;
impl<'a, const O: u8> AES_MODE_W<'a, O> {
    #[doc = "AES Mode is ECB (Electronic Code Book)."]
    #[inline(always)]
    pub fn aes_mode_ecb(self) -> &'a mut W {
        self.variant(AES_MODE_A::AES_MODE_ECB)
    }
    #[doc = "AES Mode is CBC (Cipher Block Chaining)."]
    #[inline(always)]
    pub fn aes_mode_cbc(self) -> &'a mut W {
        self.variant(AES_MODE_A::AES_MODE_CBC)
    }
}
#[doc = "Field `AES_REQUEST` reader - Request AES Operation."]
pub type AES_REQUEST_R = crate::BitReader<bool>;
#[doc = "Field `AES_REQUEST` writer - Request AES Operation."]
pub type AES_REQUEST_W<'a, const O: u8> = crate::BitWriter<'a, u8, AES_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - AES Interrupt Enable"]
    #[inline(always)]
    pub fn aes_im(&self) -> AES_IM_R {
        AES_IM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set AES Operation Direction"]
    #[inline(always)]
    pub fn aes_dir(&self) -> AES_DIR_R {
        AES_DIR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Set AES Operation Mode"]
    #[inline(always)]
    pub fn aes_mode(&self) -> AES_MODE_R {
        AES_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Request AES Operation."]
    #[inline(always)]
    pub fn aes_request(&self) -> AES_REQUEST_R {
        AES_REQUEST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - AES Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aes_im(&mut self) -> AES_IM_W<2> {
        AES_IM_W::new(self)
    }
    #[doc = "Bit 3 - Set AES Operation Direction"]
    #[inline(always)]
    #[must_use]
    pub fn aes_dir(&mut self) -> AES_DIR_W<3> {
        AES_DIR_W::new(self)
    }
    #[doc = "Bit 5 - Set AES Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn aes_mode(&mut self) -> AES_MODE_W<5> {
        AES_MODE_W::new(self)
    }
    #[doc = "Bit 7 - Request AES Operation."]
    #[inline(always)]
    #[must_use]
    pub fn aes_request(&mut self) -> AES_REQUEST_W<7> {
        AES_REQUEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_ctrl](index.html) module"]
pub struct AES_CTRL_SPEC;
impl crate::RegisterSpec for AES_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [aes_ctrl::R](R) reader structure"]
impl crate::Readable for AES_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aes_ctrl::W](W) writer structure"]
impl crate::Writable for AES_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_CTRL to value 0"]
impl crate::Resettable for AES_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
