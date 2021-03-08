#include "birli/include/cxx_aoflagger.h"
#include "birli/src/lib.rs.h"
#include <aoflagger.h>

using namespace std;
using namespace aoflagger;
using namespace rust;

void aoflagger_GetVersion(short& major, short& minor, short& subMinor)
{
	AOFlagger::GetVersion(major, minor, subMinor);
}

CxxImageSet::CxxImageSet() : pImpl(new ImageSet()) {
}
CxxImageSet::CxxImageSet(ImageSet impl) : pImpl(shared_ptr<ImageSet>(new ImageSet(impl))) {
}
size_t CxxImageSet::Width() const {
	return this->pImpl->Width();
}
size_t CxxImageSet::Height() const {
	return this->pImpl->Height();
}
size_t CxxImageSet::ImageCount() const {
	return this->pImpl->ImageCount();
}
size_t CxxImageSet::HorizontalStride() const {
	return this->pImpl->HorizontalStride();
}

rust::Slice<float> CxxImageSet::ImageBuffer(size_t imageIndex) const {
	rust::Slice<float> slice{this->pImpl->ImageBuffer(imageIndex), Width() * Height()};
	return slice;
}

CxxAOFlagger::CxxAOFlagger() : pImpl(new AOFlagger()) {
}
void CxxAOFlagger::GetVersion(short& major, short& minor, short& subMinor) const {
	this->pImpl->GetVersion(major, minor, subMinor);
}
unique_ptr<CxxImageSet> CxxAOFlagger::MakeImageSet(size_t width, size_t height, size_t count, float initialValue, size_t widthCapacity) const {
	ImageSet imageset = this->pImpl->MakeImageSet(width, height, count, initialValue, widthCapacity);
	return unique_ptr<CxxImageSet>(new CxxImageSet(imageset));
}

unique_ptr<CxxAOFlagger> cxx_aoflagger_new() {
	return unique_ptr<CxxAOFlagger>(new CxxAOFlagger());
};
