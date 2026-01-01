#[doc = "Register `CTRLD` reader"]
pub struct R(crate::R<CTRLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLD` writer"]
pub struct W(crate::W<CTRLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLD_SPEC>;
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
impl From<crate::W<CTRLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ABW` reader - Auto Baud Window"]
pub type ABW_R = crate::FieldReader<u8, ABW_A>;
#[doc = "Auto Baud Window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ABW_A {
    #[doc = "0: 18% tolerance"]
    WDW0 = 0,
    #[doc = "1: 15% tolerance"]
    WDW1 = 1,
    #[doc = "2: 21% tolerance"]
    WDW2 = 2,
    #[doc = "3: 25% tolerance"]
    WDW3 = 3,
}
impl From<ABW_A> for u8 {
    #[inline(always)]
    fn from(variant: ABW_A) -> Self {
        variant as _
    }
}
impl ABW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABW_A {
        match self.bits {
            0 => ABW_A::WDW0,
            1 => ABW_A::WDW1,
            2 => ABW_A::WDW2,
            3 => ABW_A::WDW3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WDW0`"]
    #[inline(always)]
    pub fn is_wdw0(&self) -> bool {
        *self == ABW_A::WDW0
    }
    #[doc = "Checks if the value of the field is `WDW1`"]
    #[inline(always)]
    pub fn is_wdw1(&self) -> bool {
        *self == ABW_A::WDW1
    }
    #[doc = "Checks if the value of the field is `WDW2`"]
    #[inline(always)]
    pub fn is_wdw2(&self) -> bool {
        *self == ABW_A::WDW2
    }
    #[doc = "Checks if the value of the field is `WDW3`"]
    #[inline(always)]
    pub fn is_wdw3(&self) -> bool {
        *self == ABW_A::WDW3
    }
}
#[doc = "Field `ABW` writer - Auto Baud Window"]
pub type ABW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CTRLD_SPEC, u8, ABW_A, 2, O>;
impl<'a, const O: u8> ABW_W<'a, O> {
    #[doc = "18% tolerance"]
    #[inline(always)]
    pub fn wdw0(self) -> &'a mut W {
        self.variant(ABW_A::WDW0)
    }
    #[doc = "15% tolerance"]
    #[inline(always)]
    pub fn wdw1(self) -> &'a mut W {
        self.variant(ABW_A::WDW1)
    }
    #[doc = "21% tolerance"]
    #[inline(always)]
    pub fn wdw2(self) -> &'a mut W {
        self.variant(ABW_A::WDW2)
    }
    #[doc = "25% tolerance"]
    #[inline(always)]
    pub fn wdw3(self) -> &'a mut W {
        self.variant(ABW_A::WDW3)
    }
}
impl R {
    #[doc = "Bits 6:7 - Auto Baud Window"]
    #[inline(always)]
    pub fn abw(&self) -> ABW_R {
        ABW_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 6:7 - Auto Baud Window"]
    #[inline(always)]
    #[must_use]
    pub fn abw(&mut self) -> ABW_W<6> {
        ABW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrld](index.html) module"]
pub struct CTRLD_SPEC;
impl crate::RegisterSpec for CTRLD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrld::R](R) reader structure"]
impl crate::Readable for CTRLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrld::W](W) writer structure"]
impl crate::Writable for CTRLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLD to value 0"]
impl crate::Resettable for CTRLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
