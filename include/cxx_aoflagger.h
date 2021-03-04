#pragma once
#include "rust/cxx.h"
#include <memory>

// using namespace aoflagger;

class CxxAOFlagger {
public:
    CxxAOFlagger();
    void GetVersion(short& major, short& minor, short& subMinor) const;
private:
    class impl;
    // Opaque pointer to implementation details
    std::shared_ptr<impl> pImpl;
};

void aoflagger_GetVersion(short& major, short& minor, short& subMinor);
std::unique_ptr<CxxAOFlagger> cxx_aoflagger_new();
// void cxx_aoflagger_GetVersion(CxxAOFlagger& self, short& major, short& minor, short& subMinor);
// static aoflagger::ImageSet cxx_aoflagger_AOFlagger_MakeImageSet(size_t width, size_t height, size_t count);

