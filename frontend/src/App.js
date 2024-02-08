import { Route, Routes, useLocation, useNavigate } from 'react-router-dom';
import './App.scss';
import { LoginPage } from './pages/LoginPage/LoginPage';
import MainPage from './pages/MainPage/MainPage';
import FractionPage from './pages/FractionPage/FractionPage';
import { useEffect } from 'react';
import { useDispatch } from 'react-redux';
import { getCookie } from './helpers/cookies';
import { getPerson, setAvatar, setFraction, setId, setName } from './redux/status';
import { StatusPage } from './pages/StatusPage/StatusPage';

function App() {

  const dispatch = useDispatch();
  const navigator = useNavigate();
  const location = useLocation();

  console.log(location.pathname)
  useEffect(() => {
    const id = getCookie("togethernessId");
    console.log(id);
    //Если в куках есть id
    if (id !== undefined) {
      dispatch(getPerson({
        id, callback: (name, id, avatarId, fractionId, timer) => {
          dispatch(setName(name));
          dispatch(setId(id));
          dispatch(setFraction(fractionId));
          dispatch(setAvatar(avatarId));

          if (location.pathname === "/" || location.pathname === "/fraction") navigator("/main");
        },
        errorHandler: (error) => {
          console.log("Пользователь не зарегистрирован. ", error);
          if (location.pathname !== "/fraction" && location.pathname !== "/") navigator("/");
        }
      }));
      console.log("Обновление");
    }
    else {
      console.log("cookies === ?");
      if (location.pathname !== "/fraction" && location.pathname !== "/") { navigator("/"); console.log("cookies === undefinied"); }
    }
  }, [location.pathname])

  console.log(process.env.REACT_APP_ADDR);

  return (
    <div className="App">
      <Routes>
        <Route path='*' element={<LoginPage />} />
        <Route path='/*' element={<LoginPage />} />
        <Route path='/fraction' element={<FractionPage />} />
        <Route path='/main' element={<MainPage />} />
        <Route path='/status' element={<StatusPage />} />
      </Routes>
    </div>
  );
}

export default App;