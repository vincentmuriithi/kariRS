#[doc = "Register `XMCRB` reader"]
pub struct R(crate::R<XMCRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XMCRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XMCRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XMCRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XMCRB` writer"]
pub struct W(crate::W<XMCRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XMCRB_SPEC>;
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
impl From<crate::W<XMCRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XMCRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XMM` reader - External Memory High Mask"]
pub type XMM_R = crate::FieldReader<u8, XMM_A>;
#[doc = "External Memory High Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XMM_A {
    #[doc = "0: None"]
    VAL_0X00 = 0,
    #[doc = "1: Px7"]
    VAL_0X01 = 1,
    #[doc = "2: Px7-Px6"]
    VAL_0X02 = 2,
    #[doc = "3: Px7-Px5"]
    VAL_0X03 = 3,
    #[doc = "4: Px7-Px4"]
    VAL_0X04 = 4,
    #[doc = "5: Px7-Px3"]
    VAL_0X05 = 5,
    #[doc = "6: Px7-Px2"]
    VAL_0X06 = 6,
    #[doc = "7: Full Port X"]
    VAL_0X07 = 7,
}
impl From<XMM_A> for u8 {
    #[inline(always)]
    fn from(variant: XMM_A) -> Self {
        variant as _
    }
}
impl XMM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XMM_A {
        match self.bits {
            0 => XMM_A::VAL_0X00,
            1 => XMM_A::VAL_0X01,
            2 => XMM_A::VAL_0X02,
            3 => XMM_A::VAL_0X03,
            4 => XMM_A::VAL_0X04,
            5 => XMM_A::VAL_0X05,
            6 => XMM_A::VAL_0X06,
            7 => XMM_A::VAL_0X07,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VAL_0X00`"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == XMM_A::VAL_0X00
    }
    #[doc = "Checks if the value of the field is `VAL_0X01`"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == XMM_A::VAL_0X01
    }
    #[doc = "Checks if the value of the field is `VAL_0X02`"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == XMM_A::VAL_0X02
    }
    #[doc = "Checks if the value of the field is `VAL_0X03`"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == XMM_A::VAL_0X03
    }
    #[doc = "Checks if the value of the field is `VAL_0X04`"]
    #[inline(always)]
    pub fn is_val_0x04(&self) -> bool {
        *self == XMM_A::VAL_0X04
    }
    #[doc = "Checks if the value of the field is `VAL_0X05`"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == XMM_A::VAL_0X05
    }
    #[doc = "Checks if the value of the field is `VAL_0X06`"]
    #[inline(always)]
    pub fn is_val_0x06(&self) -> bool {
        *self == XMM_A::VAL_0X06
    }
    #[doc = "Checks if the value of the field is `VAL_0X07`"]
    #[inline(always)]
    pub fn is_val_0x07(&self) -> bool {
        *self == XMM_A::VAL_0X07
    }
}
#[doc = "Field `XMM` writer - External Memory High Mask"]
pub type XMM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, XMCRB_SPEC, u8, XMM_A, 3, O>;
impl<'a, const O: u8> XMM_W<'a, O> {
    #[doc = "None"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut W {
        self.variant(XMM_A::VAL_0X00)
    }
    #[doc = "Px7"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut W {
        self.variant(XMM_A::VAL_0X01)
    }
    #[doc = "Px7-Px6"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut W {
        self.variant(XMM_A::VAL_0X02)
    }
    #[doc = "Px7-Px5"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut W {
        self.variant(XMM_A::VAL_0X03)
    }
    #[doc = "Px7-Px4"]
    #[inline(always)]
    pub fn val_0x04(self) -> &'a mut W {
        self.variant(XMM_A::VAL_0X04)
    }
    #[doc = "Px7-Px3"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut W {
        self.variant(XMM_A::VAL_0X05)
    }
    #[doc = "Px7-Px2"]
    #[inline(always)]
    pub fn val_0x06(self) -> &'a mut W {
        self.variant(XMM_A::VAL_0X06)
    }
    #[doc = "Full Port X"]
    #[inline(always)]
    pub fn val_0x07(self) -> &'a mut W {
        self.variant(XMM_A::VAL_0X07)
    }
}
#[doc = "Field `XMBK` reader - External Memory Bus Keeper Enable"]
pub type XMBK_R = crate::BitReader<bool>;
#[doc = "Field `XMBK` writer - External Memory Bus Keeper Enable"]
pub type XMBK_W<'a, const O: u8> = crate::BitWriter<'a, u8, XMCRB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - External Memory High Mask"]
    #[inline(always)]
    pub fn xmm(&self) -> XMM_R {
        XMM_R::new(self.bits & 7)
    }
    #[doc = "Bit 7 - External Memory Bus Keeper Enable"]
    #[inline(always)]
    pub fn xmbk(&self) -> XMBK_R {
        XMBK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - External Memory High Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xmm(&mut self) -> XMM_W<0> {
        XMM_W::new(self)
    }
    #[doc = "Bit 7 - External Memory Bus Keeper Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xmbk(&mut self) -> XMBK_W<7> {
        XMBK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Memory Control Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xmcrb](index.html) module"]
pub struct XMCRB_SPEC;
impl crate::RegisterSpec for XMCRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [xmcrb::R](R) reader structure"]
impl crate::Readable for XMCRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xmcrb::W](W) writer structure"]
impl crate::Writable for XMCRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XMCRB to value 0"]
impl crate::Resettable for XMCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
