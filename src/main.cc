#include "common.h"
#include "utils.h"

int main()
{
    if (sodium_init() == -1)
    {
        return 1;
    }

    BOOST_LOG_TRIVIAL(info) << "Arete(" << ARETE_GIT_VERSION << " " << __TIMESTAMP__ ") Boost(v" << (BOOST_VERSION / 100000) << "." << ((BOOST_VERSION / 100) % 1000) << "." << (BOOST_VERSION % 100) << ")";

    BOOST_LOG_TRIVIAL(warning) << "Exit.";

    return EXIT_SUCCESS;
}