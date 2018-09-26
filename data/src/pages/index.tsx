import { graphql } from "gatsby";
import * as React from "react";
import ReactTable from "react-table";
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
interface IResultSummed extends IResult {
  greedy: number;
  random: number;
  g3: number;
}
interface IndexPageProps {
  data: {
    site: {
      siteMetadata: {
        siteName: string;
      };
    };
    greedyJson: {
      r: IResult[];
    };
    greedy3Json: {
      r: IResult[];
    };
    randomJson: {
      r: IResult[];
    };
  };
}

export const pageQuery = graphql`
  query IndexQuery {
    greedyJson {
      r {
        time
        weight
        profit
        name
        index
      }
    }

    greedy3Json {
      r {
        time
        weight
        profit
        name
        index
      }
    }

    randomJson {
      r {
        time
        weight
        profit
        name
        index
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
    throw Error("Nao encontrado no greedy2");
  }
};

const MakeCell = (props: { value: number; row: IResultSummed }) => {
  const sorted = [props.row.random, props.row.greedy, props.row.g3].sort(
    (a, b) => a - b
  );
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
    const data = this.props.data.greedyJson.r.map(
      (item): IResultSummed => ({
        ...item,
        greedy: item.profit,
        random: getProfitFromFile(this.props.data.randomJson.r, item),
        g3: getProfitFromFile(this.props.data.greedy3Json.r, item)
      })
    );

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
      {
        Header: "Greedy Closest City Profit",
        accessor: "greedy",
        Cell: MakeCell
      },
      {
        Header: "Greedy Test Profit",
        accessor: "g3",
        Cell: MakeCell
      },
      {
        Header: "Choose Random City Profit",
        accessor: "random",
        Cell: MakeCell
      }
    ];
    return (
      <ReactTable
        defaultPageSize={200}
        data={data.map(x => ({
          ...x,
          name: getNameFromPath(x.name)
        }))}
        columns={columns}
      />
    );
  }
}
