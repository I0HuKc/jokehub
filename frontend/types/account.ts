export type Auth = {
  access_token: string;
  refresh_token: string;
};

export type Strength = {
  throttling_100_hour: string;
  throttling_10_second: string;
  throttling_10k_second: string;
  throttling_10b_second: string;
};
