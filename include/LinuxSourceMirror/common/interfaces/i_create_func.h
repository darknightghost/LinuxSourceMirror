/**
 * Copyright 2021, 王思远 <darknightghost.cn@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free Software
 * Foundation, either version 3 of the License, or at your option) any later
 * version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
 * details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program.  If not, see <http://www.gnu.org/licenses/>.
 */
#pragma once

#include <type_traits>

#include <LinuxSourceMirror/common/interfaces/i_initialized.h>
#include <LinuxSourceMirror/common/interfaces/i_this_ptr.h>

namespace common {

/**
 * @brief   Interface to provide a static create method.
 *
 * @tparam  T       Final class which implements this interface.
 * @tparam  Args    Types of arguments of the constructor of the class which
 *                  implements this interface.
 */
template<class T, typename... Args>
class ICreateFunc : virtual public IInitialized<T>, virtual public IThisPtr<T> {
  protected:
    /**
     * @brief       Constructor.
     */
    ICreateFunc();

  public:
    /**
     * @brief       Create object.
     *
     * @param[in]   args    Arguments of the constructor.
     *
     * @return      On success, the new object is returned. Otherwise returns
     *              \c nullptr.
     */
    static inline ::std::shared_ptr<T> create(Args... args);

    /**
     * @brief       Destructor.
     */
    virtual ~ICreateFunc();
};

/**
 * @brief     Constructor.
 */
template<class T, typename... Args>
ICreateFunc<T, Args...>::ICreateFunc()
{
    static_assert(::std::is_base_of<ICreateFunc<T, Args...>, T>::value,
                  "ICreateFunc<T, Args...> is not a base class of T.");
}

/**
 * @brief     Create object.
 */
template<class T, typename... Args>
::std::shared_ptr<T> ICreateFunc<T, Args...>::create(Args... args)
{
    // Create object.
    ::std::shared_ptr<T> ret
        = ::std::make_shared<T>(::std::forward<Args...>(args)...);

    // Check result.
    if (ret == nullptr) {
        return ret;

    } else if (! ret->IInitialized<T>::initialized()) {
        return nullptr;

    } else {
        ret->setThisPtr(ret);
        return ret;
    }
}

/**
 * @brief       Destructor.
 */
template<class T, typename... Args>
ICreateFunc<T, Args...>::~ICreateFunc()
{}

#define CREATE_FUNC(T, ...)                               \
    friend class ::common::ICreateFunc<T, ##__VA_ARGS__>; \
    friend ::std::shared_ptr<T>::std::make_shared(Args...);

} // namespace common
