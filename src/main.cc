#include "common.h"
#include "utils.h"

int main()
{

    BOOST_LOG_TRIVIAL(info) << "Arete(" << ARETE_VERSION << ") Boost(v" << (BOOST_VERSION / 100000) << "." << ((BOOST_VERSION / 100) % 1000) << "." << (BOOST_VERSION % 100) << ")";

    BOOST_LOG_TRIVIAL(warning) << "Exit.";

    return 0;
}