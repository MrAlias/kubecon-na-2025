import http from "k6/http";

export const options = {
  scenarios: {
    constant_load: {
      executor: "constant-arrival-rate",
      rate: 5, // 5 requests per second
      timeUnit: "1s", // per second
      duration: "24h", // run forever
      preAllocatedVUs: 10, // pre-allocate VUs
      maxVUs: 20, // maximum VUs if needed
    },
  },
};

export default function () {
  http.get("http://localhost:9080/productpage");
}
