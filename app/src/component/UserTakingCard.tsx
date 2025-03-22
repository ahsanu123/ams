import type { Product, User } from '@/model'
import './UserTakingCard.css'

interface UserTakingCardProps {
  data: Product,
  user: User
}
export default function UserTakingCard(props: UserTakingCardProps) {
  const {
    data,
    user
  } = props
  return (
    <div
      className="user-taking-card"
    >
      <label>
        {user.username}
      </label>
    </div>
  )
}
