#[doc = "Register `MAN_ID_1` reader"]
pub struct R(crate::R<MAN_ID_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAN_ID_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAN_ID_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAN_ID_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAN_ID_1` writer"]
pub struct W(crate::W<MAN_ID_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAN_ID_1_SPEC>;
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
impl From<crate::W<MAN_ID_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAN_ID_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAN_ID_` reader - Manufacturer ID (High Byte)"]
pub type MAN_ID__R = crate::FieldReader<u8, MAN_ID__A>;
#[doc = "Manufacturer ID (High Byte)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAN_ID__A {
    #[doc = "0: Atmel JEDEC manufacturer ID, bits \\[15:8\\]
of 32 bit manufacturer ID: 00 00 00 1F"]
    ATMEL_BYTE_1 = 0,
}
impl From<MAN_ID__A> for u8 {
    #[inline(always)]
    fn from(variant: MAN_ID__A) -> Self {
        variant as _
    }
}
impl MAN_ID__R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAN_ID__A> {
        match self.bits {
            0 => Some(MAN_ID__A::ATMEL_BYTE_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ATMEL_BYTE_1`"]
    #[inline(always)]
    pub fn is_atmel_byte_1(&self) -> bool {
        *self == MAN_ID__A::ATMEL_BYTE_1
    }
}
#[doc = "Field `MAN_ID_` writer - Manufacturer ID (High Byte)"]
pub type MAN_ID__W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, MAN_ID_1_SPEC, u8, MAN_ID__A, 8, O>;
impl<'a, const O: u8> MAN_ID__W<'a, O> {
    #[doc = "Atmel JEDEC manufacturer ID, bits \\[15:8\\]
of 32 bit manufacturer ID: 00 00 00 1F"]
    #[inline(always)]
    pub fn atmel_byte_1(self) -> &'a mut W {
        self.variant(MAN_ID__A::ATMEL_BYTE_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Manufacturer ID (High Byte)"]
    #[inline(always)]
    pub fn man_id_(&self) -> MAN_ID__R {
        MAN_ID__R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Manufacturer ID (High Byte)"]
    #[inline(always)]
    #[must_use]
    pub fn man_id_(&mut self) -> MAN_ID__W<0> {
        MAN_ID__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Identification Register (Manufacture ID High Byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [man_id_1](index.html) module"]
pub struct MAN_ID_1_SPEC;
impl crate::RegisterSpec for MAN_ID_1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [man_id_1::R](R) reader structure"]
impl crate::Readable for MAN_ID_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [man_id_1::W](W) writer structure"]
impl crate::Writable for MAN_ID_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAN_ID_1 to value 0"]
impl crate::Resettable for MAN_ID_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
