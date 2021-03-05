#pragma once
#include "rust/cxx.h"
#include <memory>
#include <aoflagger.h>

using namespace std;
using namespace aoflagger;

class CxxImageSet {
friend class CxxAOFlagger;
public:
size_t Width() const;
size_t Height() const;
size_t ImageCount() const;
size_t HorizontalStride() const;
rust::Slice<float> ImageBuffer(size_t imageIndex) const;
private:
CxxImageSet();
CxxImageSet(ImageSet impl);
shared_ptr<ImageSet> pImpl;
};

class CxxAOFlagger {
public:
CxxAOFlagger();
void GetVersion(short& major, short& minor, short& subMinor) const;
unique_ptr<CxxImageSet> MakeImageSet(size_t width, size_t height, size_t count, float initialValue, size_t widthCapacity) const;
private:
// Opaque pointer to aoflagger implementation
shared_ptr<AOFlagger> pImpl;
};

void aoflagger_GetVersion(short& major, short& minor, short& subMinor);
unique_ptr<CxxAOFlagger> cxx_aoflagger_new();
// void cxx_aoflagger_GetVersion(CxxAOFlagger& self, short& major, short& minor, short& subMinor);
// static ImageSet cxx_aoflagger_AOFlagger_MakeImageSet(size_t width, size_t height, size_t count);

