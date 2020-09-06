#include "utils.h"

std::string arete::utils::random::uuid_v4()
{
    boost::uuids::random_generator generator;
    boost::uuids::uuid uuid = generator();
    return boost::uuids::to_string(uuid);
}