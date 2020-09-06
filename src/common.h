#ifndef ARETE_COMMON_H_
#define ARETE_COMMON_H_

#include <cassert>
#include <cstdlib>
#include <exception>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <string>

#include <amqp.h>
#include <boost/foreach.hpp>
#include <boost/log/trivial.hpp>
#include <boost/property_tree/json_parser.hpp>
#include <boost/property_tree/ptree.hpp>
#include <boost/property_tree/xml_parser.hpp>
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>
#include <hiredis/hiredis.h>
#include <mysql++/mysql++.h>
#include <pqxx/pqxx>
#include <sodium.h>
#include <sqlite3.h>

#include "config.h"

#endif