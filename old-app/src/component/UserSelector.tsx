import { useMainLayoutStore } from '@/state';
import { type ChangeEventHandler } from 'react';

export default function UserSelector() {

  const listUser = useMainLayoutStore(state => state.listUser);
  const setUser = useMainLayoutStore(state => state.setCustomer)
  const selectedUser = useMainLayoutStore(state => state.customer)

  const handleOnSelectChange: ChangeEventHandler<HTMLSelectElement> = ({ target }) => {
    const value = parseInt(target.value)
    if (value != -1) {
      const user = listUser.find((user) => user.id === value)
      user && setUser(user)
    }
    else setUser(undefined)
  }

  return (
    <>
      <select
        onChange={handleOnSelectChange}
        value={selectedUser?.id ?? -1}
      >
        <option
          key={-1}
          value={-1}
        >
          Pilih Nama Pengguna
        </option>
        {listUser.map((user, index) => (
          <option
            key={index}
            value={user.id}
          >
            {user.username}
          </option>
        ))}
      </select>
    </>
  )
}
