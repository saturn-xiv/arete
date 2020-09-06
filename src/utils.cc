#include "utils.h"

std::string arete::utils::random::uuid_v4()
{
    boost::uuids::random_generator generator;
    boost::uuids::uuid uuid = generator();
    return boost::uuids::to_string(uuid);
}

void arete::orm::sqlite::Connection::open()
{
    //     sqlite3 *db;
    //    char *zErrMsg = 0;
    //    int rc;

    //    rc = sqlite3_open("test.db", &db);

    //    if( rc ) {
    //       fprintf(stderr, "Can't open database: %s\n", sqlite3_errmsg(db));
    //       return(0);
    //    } else {
    //       fprintf(stderr, "Opened database successfully\n");
    //    }
    //    sqlite3_close(db);
}

void arete::orm::postgresql::Connection::open()
{
    // pqxx::connection c{"postgresql://accounting@localhost/company"};
    // pqxx::work txn{c};

    // // Normally we'd query the DB using txn.exec().  But for querying just one
    // // single value, we can use txn.query_value() as a shorthand.
    // //
    // // Use txn.quote() to escape and quote a C++ string for use as an SQL string
    // // in a query's text.
    // int employee_id = txn.query_value<int>(
    //     "SELECT id "
    //     "FROM Employee "
    //     "WHERE name =" +
    //     txn.quote(argv[1]));

    // std::cout << "Updating employee #" << employee_id << '\n';

    // // Update the employee's salary.  Use exec0() to perform a query and check
    // // that it produces an empty result.  If the result does contain data, it
    // // will throw an exception.
    // //
    // // The ID is an integer, so we don't need to escape and quote it when using
    // // it in our query text.  Just convert it to its PostgreSQL string
    // // representation using to_string().
    // txn.exec0(
    //     "UPDATE EMPLOYEE "
    //     "SET salary = salary + 1 "
    //     "WHERE id = " +
    //     pqxx::to_string(employee_id));

    // // Make our change definite.
    // txn.commit();
}

void arete::orm::mysql::Connection::open()
{
    // mysqlpp::examples::CommandLine cmdline(1, [""]);
    // if (!cmdline)
    // {
    //     return 1;
    // }

    // // Connect to the sample database.
    // mysqlpp::Connection conn(false);
    // if (conn.connect(mysqlpp::examples::db_name, cmdline.server(),
    //                  cmdline.user(), cmdline.pass()))
    // {
    //     // Retrieve a subset of the sample stock table set up by resetdb
    //     // and display it.
    //     mysqlpp::Query query = conn.query("select item from stock");
    //     if (mysqlpp::StoreQueryResult res = query.store())
    //     {
    //         cout << "We have:" << endl;
    //         mysqlpp::StoreQueryResult::const_iterator it;
    //         for (it = res.begin(); it != res.end(); ++it)
    //         {
    //             mysqlpp::Row row = *it;
    //             cout << '\t' << row[0] << endl;
    //         }
    //     }
    //     else
    //     {
    //         cerr << "Failed to get item list: " << query.error() << endl;
    //         return 1;
    //     }

    //     return 0;
}