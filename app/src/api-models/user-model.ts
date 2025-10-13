export interface UserModel {
  id?: number;
  username: string;
  isActive: boolean;
  isAdmin: boolean;
  money: bigint;
  createdDate: Date;
  updatedDate: Date;
}
