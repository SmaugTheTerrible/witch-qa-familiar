import * as React from "react";
import styles from "./Hello.module.css";
import { useAppDispatch, useAppSelector } from "../../app/hooks";
import { fetchHello } from "./helloSlice";

interface IHelloProps {}

const Hello: React.FunctionComponent<IHelloProps> = (props) => {
  const { response } = useAppSelector((state) => state.hello);
  const dispatch = useAppDispatch();

  React.useEffect(() => {
    dispatch(fetchHello("Boris"))
  }, [dispatch]);

  return (
    <div className={styles.hello}>
      <p>{response}</p>
    </div>
  );
};

export default Hello;
