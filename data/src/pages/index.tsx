import { graphql } from "gatsby";
import * as React from "react";
import ReactTable from "react-table";
// tslint:disable-next-line:no-submodule-imports
import "react-table/react-table.css";

// Please note that you can use https://github.com/dotansimha/graphql-code-generator
// to generate all types from graphQL schema
interface IResult {
  time: number;
  weight: number;
  profit: number;
  index: number;
  name: string;
}
// tslint:disable-next-line:no-empty-interface
interface IResultSummed extends IResult {
  // greedy: number;
  // random: number;
  // g3: number;
}
interface IndexPageProps {
  data: {
    site: {
      siteMetadata: {
        siteName: string;
      };
    };
    allRawJson: {
      edges: Array<{
        node: {
          name: string;
          r: IResult[];
        };
      }>;
    };
  };
}

export const pageQuery = graphql`
  query IndexQuery {
    allRawJson {
      edges {
        node {
          name
          r {
            time
            weight
            profit
            name
            index
          }
        }
      }
    }
  }
`;

const getNameFromPath = (name: string) => {
  const x = name.split("/");
  return x[x.length - 1];
};
const getProfitFromFile = (data: IResult[], item: IResult) => {
  const d = data.find(x => x.index === item.index);

  if (d) {
    return d.profit;
  } else {
    // throw Error("Nao encontrado no greedy2");
    return -1;
  }
};

const MakeCell = (names: string[]) => (props: {
  value: number;
  row: IResultSummed;
}) => {
  const values = names.map(name => (props.row as any)[name]);

  const sorted = values.sort((a, b) => a - b);
  const min = sorted[0];
  const max = sorted[sorted.length - 1];
  const total = max - min;
  const frac = props.value - min;
  const unitary = Math.abs(frac / total);
  const hex = (255 - Math.floor(Math.pow(unitary, 3) * 60)).toString(16);
  const color = `#${hex}ff${hex}`;
  return (
    <span style={{ backgroundColor: color }} className="number">
      {props.value}
    </span>
  ); // Custom cell components!
};
export default class IndexPage extends React.Component<IndexPageProps, {}> {
  public render() {
    const dataMap = this.props.data.allRawJson.edges
      .map(item => item.node)
      .reduce((prev, curr) => {
        return {
          ...prev,
          [curr.name]: curr.r
        };
      }, {});
    const names = Object.keys(dataMap);
    const x = (dataMap as any)[names[0]] as IResult[];
    const data = x
      .map(
        (item): any => {
          const toAdd = names.reduce((prev, name) => {
            return {
              ...prev,
              [name]: getProfitFromFile(
                (dataMap as any)[name] as IResult[],
                item
              )
            };
          }, {});
          return {
            ...item,
            ...toAdd
          };
        }
      )
      .map(item => {
        return { ...item, name: getNameFromPath(item.name) };
      });

    const columnsToAdd = names.map(name => {
      return {
        Cell: MakeCell(names),
        Header: name,
        accessor: name
      };
    });

    const columns = [
      {
        Header: "Index",
        accessor: "index",
        width: 64
      },
      {
        Header: "Name",
        accessor: "name"
      },
      ...columnsToAdd
    ];
    return <ReactTable defaultPageSize={200} data={data} columns={columns} />;
  }
}
