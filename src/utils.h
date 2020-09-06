#ifndef ARETE_UTILS_H_
#define ARETE_UTILS_H_

#include "common.h"

namespace arete
{
    namespace orm
    {
        namespace mysql
        {
            class Connection
            {
            public:
                void open();

            private:
                mysqlpp::Connection *con;
            };
        } // namespace mysql
        namespace sqlite
        {
            class Connection
            {
            public:
                void open();

            private:
                sqlite3 *con;
            };
        } // namespace sqlite
        namespace postgresql
        {
            class Connection
            {
            public:
                void open();

            private:
                pqxx::connection *con;
            };
        } // namespace postgresql
    }     // namespace orm
    namespace utils
    {
        namespace random
        {
            std::string uuid_v4();
        }
    } // namespace utils
} // namespace arete
#endif