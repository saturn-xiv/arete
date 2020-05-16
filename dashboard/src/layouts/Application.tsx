import { IRouteComponentProps } from 'umi';

export default function({
  children,
  location,
  route,
  history,
  match,
}: IRouteComponentProps) {
  return (
    <div>
      <h1>application header</h1>
      {children}
      <h2>application footer</h2>
    </div>
  );
}
