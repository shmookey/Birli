#include <aoflagger.h>

extern "C" {
    static void aoflagger_AOFlagger_GetVersion(short& major, short& minor, short& subMinor) {
        aoflagger::AOFlagger::GetVersion(major, minor, subMinor);
    }
}
