#include "birli/include/cxx_aoflagger.h"
#include "birli/src/lib.rs.h"
#include <aoflagger.h>

// using namespace aoflagger;

// auto aoflaggerGlbl = new AOFlagger();

void aoflagger_GetVersion(short& major, short& minor, short& subMinor)
{
	aoflagger::AOFlagger::GetVersion(major, minor, subMinor);
}

class CxxAOFlagger::impl {
friend CxxAOFlagger;
private:
std::shared_ptr<aoflagger::AOFlagger> aoflagger;
impl() : aoflagger(new aoflagger::AOFlagger()) {}
};

CxxAOFlagger::CxxAOFlagger() : pImpl(new class CxxAOFlagger::impl) {
}
void CxxAOFlagger::GetVersion(short& major, short& minor, short& subMinor) const {
    this->pImpl->aoflagger->GetVersion(major, minor, subMinor);
}

std::unique_ptr<CxxAOFlagger> cxx_aoflagger_new() {
	return std::unique_ptr<CxxAOFlagger>(new CxxAOFlagger());
};


// This is required because the compiler inlines it by default.
// const ImageSet cxx_aoflagger_MakeImageSet(size_t width, size_t height, size_t count)
// {
// 	return aoflaggerGlbl->MakeImageSet(width, height, count);
// }
