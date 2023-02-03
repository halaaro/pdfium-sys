/// Workaround for build problems on Windows.
/// Removes access to FPDF_RenderPage and FPDF_RenderPage.
#ifdef _WIN32
#undef _WIN32
#endif

#include "fpdfview.h"
