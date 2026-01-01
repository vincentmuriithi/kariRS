#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - SPI Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "SPI Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: SPI Mode 0"]
    _0 = 0,
    #[doc = "1: SPI Mode 1"]
    _1 = 1,
    #[doc = "2: SPI Mode 2"]
    _2 = 2,
    #[doc = "3: SPI Mode 3"]
    _3 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::_0,
            1 => MODE_A::_1,
            2 => MODE_A::_2,
            3 => MODE_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == MODE_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == MODE_A::_3
    }
}
#[doc = "Field `MODE` writer - SPI Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CTRLB_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "SPI Mode 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE_A::_0)
    }
    #[doc = "SPI Mode 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE_A::_1)
    }
    #[doc = "SPI Mode 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE_A::_2)
    }
    #[doc = "SPI Mode 3"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE_A::_3)
    }
}
#[doc = "Field `SSD` reader - Slave Select Disable"]
pub type SSD_R = crate::BitReader<bool>;
#[doc = "Field `SSD` writer - Slave Select Disable"]
pub type SSD_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `BUFWR` reader - Buffer Write Mode"]
pub type BUFWR_R = crate::BitReader<bool>;
#[doc = "Field `BUFWR` writer - Buffer Write Mode"]
pub type BUFWR_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `BUFEN` reader - Buffer Mode Enable"]
pub type BUFEN_R = crate::BitReader<bool>;
#[doc = "Field `BUFEN` writer - Buffer Mode Enable"]
pub type BUFEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - SPI Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Slave Select Disable"]
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Buffer Write Mode"]
    #[inline(always)]
    pub fn bufwr(&self) -> BUFWR_R {
        BUFWR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Buffer Mode Enable"]
    #[inline(always)]
    pub fn bufen(&self) -> BUFEN_R {
        BUFEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - Slave Select Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ssd(&mut self) -> SSD_W<2> {
        SSD_W::new(self)
    }
    #[doc = "Bit 6 - Buffer Write Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bufwr(&mut self) -> BUFWR_W<6> {
        BUFWR_W::new(self)
    }
    #[doc = "Bit 7 - Buffer Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bufen(&mut self) -> BUFEN_W<7> {
        BUFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
