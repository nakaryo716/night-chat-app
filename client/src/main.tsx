import React from 'react'
import ReactDOM from 'react-dom/client'
// import { App } from './App'
import { createBrowserRouter, redirect, RouterProvider } from 'react-router-dom'
import { Root } from './components/Root'
import Chat from './components/Chat'
import { RoomList } from './components/RoomList'
import { UserName } from './components/UserName'
import { User } from './types/user'
import { getUserNameApi } from './api/userApi'
import { NewRoom } from './components/NewRoom'


const getUserNameHandler = async () => {
  try {
      const response = await getUserNameApi();

      if (!response.ok) {
          throw new Error("get user name error");
      }

      const user: User = await response.json();
      return user.user_name;
  } catch {
    return redirect("/user-name");
  }
}

const route = createBrowserRouter([
  {
    path: "/",
    element: <Root></Root>
  },
  {
    path: "/room-list",
    element: <RoomList />
  },
  {
    path: "/user-name",
    element: <UserName />
  },
  {
    path: "/create-room",
    element: <NewRoom />,
  },
  {
    path: "/chat/:chat-id",
    element: <Chat></Chat>,
    loader: () => {
      return getUserNameHandler();
    }
  },
])

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <RouterProvider router={route} />
  </React.StrictMode>,
)
