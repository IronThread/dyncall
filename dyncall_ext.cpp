

#include <cstddef>
#include <cstring>
#include "dyncall/dyncall/dyncall.h"
#include "dyncall/dynload/dynload.h"
#include "dyncall/dyncall/dyncall_struct.h"
#include "dyncall/dyncall/dyncall_macros.h"
#include "dyncall/dyncall/dyncall_alloc.h"

#if defined(DC__Arch_Intel_x86)
#  include "dyncall/dyncall/dyncall_callvm_x86.h"
const size_t VM_DATA_SIZE = sizeof(DCCallVM_x86);
#elif defined(DC__Arch_AMD64)
#  include "dyncall/dyncall/dyncall_callvm_x64.h"
const size_t VM_DATA_SIZE = sizeof(DCCallVM_x64);
#elif defined(DC__Arch_PPC32)
#  include "dyncall/dyncall/dyncall_callvm_ppc32.h"
const size_t VM_DATA_SIZE = sizeof(DCCallVM_ppc32);
#elif defined(DC__Arch_PPC64)
#  include "dyncall/dyncall/dyncall_callvm_ppc64.h"
const size_t VM_DATA_SIZE = sizeof(DCCallVM_ppc64);
#elif defined(DC__Arch_MIPS) || defined(DC__Arch_MIPS64)
#  if defined(DC__ABI_MIPS_EABI)
#    include "dyncall/dyncall/dyncall_callvm_mips_eabi.h"
const size_t VM_DATA_SIZE = sizeof(DCCallVM_mips_eabi);
#  elif defined(DC__ABI_MIPS_O32)
#    include "dyncall/dyncall/dyncall_callvm_mips_o32.h"
const size_t VM_DATA_SIZE = sizeof(DCCallVM_mips_o32);
#  elif defined(DC__ABI_MIPS_N64)
#    include "dyncall/dyncall/dyncall_callvm_mips_n64.h"
const size_t VM_DATA_SIZE = sizeof(DCCallVM_mips_n64);
#  elif defined(DC__ABI_MIPS_N32)
#    include "dyncall/dyncall/dyncall_callvm_mips_n32.h"
const size_t VM_DATA_SIZE = sizeof(DCCallVM_mips_n32);
#  else
#    error Unknown MIPS ABI.
#  endif /* DC__Arch_MIPS || DC__Arch_MIPS64 */
#elif defined(DC__Arch_ARM_ARM)
#  if defined(DC__ABI_ARM_HF)
#    include "dyncall/dyncall/dyncall_callvm_arm32_arm_armhf.h"
const size_t VM_DATA_SIZE = sizeof(DCCallVM_arm32_armhf);
#  else
#    include "dyncall/dyncall/dyncall_callvm_arm32_arm.h"
const size_t VM_DATA_SIZE = sizeof(DCCallVM_arm32_arm);
#  endif
#elif defined(DC__Arch_ARM_THUMB)
#  if defined(DC__ABI_ARM_HF)
#    include "dyncall/dyncall/dyncall_callvm_arm32_arm_armhf.h"
const size_t VM_DATA_SIZE = sizeof(DCCallVM_arm32_armhf);
#  else
#    include "dyncall/dyncall/dyncall_callvm_arm32_thumb.h"
const size_t VM_DATA_SIZE = sizeof(DCCallVM_arm32_thumb);
#  endif
#elif defined(DC__Arch_ARM64)
#    include "dyncall/dyncall/dyncall_callvm_arm64.h"
const size_t VM_DATA_SIZE = sizeof(DCCallVM_arm64);
#elif defined(DC__Arch_Sparc)
#  include "dyncall/dyncall/dyncall_callvm_sparc.h"
const size_t VM_DATA_SIZE = sizeof(DCCallVM_sparc);
#elif defined(DC__Arch_Sparc64)
#  include "dyncall/dyncall/dyncall_callvm_sparc64.h"
const size_t VM_DATA_SIZE = sizeof(DCCallVM_v9);
#elif defined(DC__Arch_RiscV)
#  include "dyncall/dyncall/dyncall_callvm_riscv.h"
const size_t VM_DATA_SIZE = sizeof(DCCallVM_riscv);
#else
#  error unsupported platform
#endif

void dcAppendStruct(DCstruct* s, const DCstruct* s2)
{
    // the only way to the last value passed to not be overwritten it's to s2 not having fields
    // at all,including substructs,that way zero it's a good value. 
	dcSubStruct(s, s2 -> fieldCount, s2 -> alignment, 0);

    for (DCsize i = 0; i < s2 -> fieldCount; i++) {
        DCfield const *elem = s2 -> pFields + i;

        if (elem -> type == DC_SIGCHAR_STRUCT) {
            dcAppendStruct(s, elem -> pSubStruct);
        } else {
            dcStructField(s, elem -> type, elem -> alignment, elem -> arrayLength);
        }
    }

    dcCloseStruct(s);
}

size_t inline vm_size() {
	return VM_DATA_SIZE;
}

size_t inline vm_align() {
    return alignof(std::max_align_t);
}

int inline unsupported_mode() {
    return DC_ERROR_UNSUPPORTED_MODE;
}

void inline freeMem(void *ptr) {
    return dcFreeMem(ptr);
}