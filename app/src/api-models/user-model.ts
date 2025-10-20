export interface UserModel {
  id?: number;
  username: string;
  isActive: boolean;
  isAdmin: boolean;
  money: number;
  createdDate: Date;
  updatedDate: Date;
}
