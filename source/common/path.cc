#include <errno.h>
#include <memory>
#include <sstream>

#include <unistd.h>

#include <LinuxSourceMirror/common/path.h>

namespace common {

///< Path seperator.
const char pathSep = '/';

/**
 * @brief       Split path.
 */
::std::vector<::std::string> splitPath(const ::std::string &path)
{
    ::std::vector<::std::string> ret;
    ::std::ostringstream         ss;

    for (auto &ch : path) {
        if (ch != pathSep) {
            ss << ch;
        } else {
            ret.push_back(ss.str());
            ss.str("");
        }
    }
    if (ss.str() != "") {
        ret.push_back(ss.str());
    }

    return ret;
}

/**
 * @brief       Join path.
 */
::std::string joinPath(const ::std::vector<::std::string> &path)
{
    ::std::ostringstream ss;
    bool                 begin = true;
    for (auto &s : path) {
        if (begin) {
            begin = false;
        } else {
            ss << pathSep;
        }
        ss << s;
    }

    return ss.str();
}

/**
 * @brief       Get absolute path.
 */
::std::vector<::std::string> abspath(const ::std::vector<::std::string> &path)
{
    ::std::vector<::std::string> ret;

    if (! (path.size() > 0 && path.front() == "")) {
        size_t                    sz = 512;
        ::std::unique_ptr<char[]> buffer;
        while (true) {
            buffer = ::std::unique_ptr<char[]>(new char[sz]);
            if (::getcwd(buffer.get(), sz) == NULL) {
                sz *= 2;
            }
        }

        ret = splitPath(buffer.get());
    }

    for (auto &s : path) {
        if (s == "") {
            ret.clear();
            ret.push_back("");

        } else if (s == ".") {
            continue;

        } else if (s == "..") {
            ret.pop_back();

        } else {
            ret.push_back(s);
        }
    }

    return ret;
}

/**
 * @brief       Get absolute path.
 */
::std::string abspath(const ::std::string &path)
{
    return joinPath(abspath(splitPath(path)));
}

/**
 * @brief       Get path of the directory of the path.
 */
::std::string dirname(const ::std::string &path)
{
    auto ret = abspath(splitPath(path));
    if (ret.size() > 1) {
        ret.pop_back();
    }

    return joinPath(ret);
}

/**
 * @brief       Create directory if not exists.
 */
int createDirectoryIfNotExists(const ::std::string &path, bool createParent) {}

} // namespace common
