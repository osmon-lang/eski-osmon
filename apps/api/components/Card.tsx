export default function Card() {
  return (
    <svg className="card" viewBox="0 50 840 440">
      <defs>
        <linearGradient
          id="gradient-1"
          gradientUnits="userSpaceOnUse"
          x1={420}
          y1={275}
          x2={420}
          y2={167}
          gradientTransform="matrix(1 0 0 -1 0 442)"
        >
          <stop
            offset={0.464}
            style={{
              stopOpacity: 0.2,
            }}
          />
          <stop
            offset={0.9}
            style={{
              stopOpacity: 0,
            }}
          />
        </linearGradient>
        <linearGradient
          id="gradient-2"
          gradientUnits="userSpaceOnUse"
          x1={420}
          y1={325}
          x2={420}
          y2={117.001}
          gradientTransform="matrix(1 0 0 -1 0 442)"
        >
          <stop
            offset={0.464}
            style={{
              stopOpacity: 0.2,
            }}
          />
          <stop
            offset={1}
            style={{
              stopOpacity: 0,
            }}
          />
        </linearGradient>
        <linearGradient
          id="gradient-3"
          gradientUnits="userSpaceOnUse"
          x1={420}
          y1={382}
          x2={420}
          y2={60.001}
          gradientTransform="matrix(1 0 0 -1 0 442)"
        >
          <stop
            offset={0.464}
            style={{
              stopOpacity: 0.16,
            }}
          />
          <stop
            offset={0.9}
            style={{
              stopOpacity: 0,
            }}
          />
        </linearGradient>
        <linearGradient
          id="gradient-4"
          gradientUnits="userSpaceOnUse"
          x1={420}
          y1={488.788}
          x2={420}
          y2={-46.967}
          gradientTransform="matrix(1 0 0 -1 0 442)"
        >
          <stop
            offset={0.089}
            style={{
              stopOpacity: 0.1,
            }}
          />
          <stop
            offset={0.464}
            style={{
              stopOpacity: 0.27,
            }}
          />
          <stop
            offset={0.896}
            style={{
              stopOpacity: 0,
            }}
          />
        </linearGradient>
        <linearGradient
          id="gradient-5"
          gradientUnits="userSpaceOnUse"
          x1={420}
          y1={610}
          x2={420}
          y2={-168.179}
          gradientTransform="matrix(1 0 0 -1 0 442)"
        >
          <stop
            offset={0.172}
            style={{
              stopOpacity: 0,
            }}
          />
          <stop
            offset={0.464}
            style={{
              stopOpacity: 0.21,
            }}
          />
          <stop
            offset={0.771}
            style={{
              stopOpacity: 0,
            }}
          />
        </linearGradient>
        <linearGradient
          id="gradient-vercel"
          gradientUnits="objectBoundingBox"
          x1={0}
          y1={0}
          x2={1.5}
          y2={1}
        >
          <stop
            offset={0.3}
            style={{
              stopColor: "var(--g1)",
            }}
          />
          <stop
            offset={0.5}
            style={{
              stopColor: "var(--g2)",
            }}
          />
          <stop
            offset={0.8}
            style={{
              stopColor: "var(--g1)",
            }}
          />
        </linearGradient>
        <linearGradient
          id="gradient-react"
          gradientUnits="objectBoundingBox"
          x1={0}
          y1={0}
          x2={1.1}
          y2={1}
        >
          <stop
            offset={0.3}
            style={{
              stopColor: "var(--react)",
            }}
          />
          <stop
            offset={0.5}
            style={{
              stopColor: "#BBF0FF",
            }}
          />
          <stop
            offset={0.8}
            style={{
              stopColor: "var(--react)",
            }}
          />
        </linearGradient>
        <symbol id="react-logo" viewBox="0 0 585.59 252.64">
          <path
            fill="var(--fg)"
            d="M339.89,471.89q17.42,21.71,17.41,54.68,0,32.38-17.41,54.07-19.56,24.14-54.07,24.13-34.81,0-54.07-24.13-17.1-21.69-17.1-54.07,0-33,17.1-54.68,19.25-24.12,54.07-24.13Q320.33,447.76,339.89,471.89ZM247,477.09q-11.92,19.55-11.92,49.48,0,29.32,11.92,48.57,13.74,22,38.79,22,24.75,0,38.49-22,12.21-19.25,12.22-48.57,0-29.93-12.22-49.48-13.74-21.69-38.49-21.69Q260.77,455.4,247,477.09Z"
            transform="translate(-214.65 -352.75)"
          />
          <path
            fill="var(--fg)"
            d="M429.39,582.78a69.07,69.07,0,0,0,42.16,14.05q15.87,0,26-8.86,10.38-8.85,10.39-23.82,0-11-9.78-18t-34.21-14.35q-35.13-11-48.26-22.61Q404,499.08,404,482.89q0-15,12.83-24.74,13.74-9.78,35.74-9.78a82.39,82.39,0,0,1,14.05,1.22q4,.93,10.39,2.75,4.88,1.23,7,1.53a8.7,8.7,0,0,0,5.19-.3l11.61-5.5L518.59,493l-5.19,2.45q-7.33-13.14-21.69-24.44-19.24-14.67-39.1-14.66-15,0-23.83,6.72-8.86,7-8.86,19.85,0,10.39,11.31,17.41,8.53,5.51,32.68,13.14,32.68,10.08,45.82,20.77,13.75,11.31,13.75,29.94,0,19.54-16.5,31.15-14.36,10.08-35.43,10.08a62.31,62.31,0,0,1-13.75-1.52,72.36,72.36,0,0,1-11-2.75c-4.27-1-7.33-1.73-9.16-2.14a15.68,15.68,0,0,0-7.64.61l-12.83,5.19-18.32-48,4-1.83Q416.56,573.63,429.39,582.78Z"
            transform="translate(-214.65 -352.75)"
          />
          <path
            fill="var(--fg)"
            d="M553.11,591.94q11.61,0,17.1-4.27,5.19-4.27,5.2-13.44V482.59q0-10.38-5.2-13.75-4.28-3-15.88-3.06l-1.22-5.49L594,447.76v28.11q13.44-12.53,23.22-18.64,16.18-9.47,33-9.47a37,37,0,0,1,21.38,6.72,35.53,35.53,0,0,1,14,20.77,112.78,112.78,0,0,1,25.05-18.93q15.59-8.55,31.16-8.56,15.89,0,26.27,11a33.1,33.1,0,0,1,9.77,23.83v91.64q0,8.87,5.5,13.44,5.8,4.27,16.8,4.27v7.64h-61.7v-7.64q10.68,0,15.58-4.27,5.19-4.27,5.19-13.44V487.47a24.5,24.5,0,0,0-7.33-18q-7.63-7.93-20.77-7.94-11.91,0-24.13,6.72-9.78,5.19-21.39,16.19v89.81q0,8.87,5.2,13.44,5.19,4.27,15.88,4.27v7.64H646.28v-7.64q11,0,15.88-4.27t4.89-13.44V486.86a24.18,24.18,0,0,0-7.33-17.72,26.34,26.34,0,0,0-18.94-7.33,50.39,50.39,0,0,0-25.66,6.72Q606,473.74,594,486.25v88q0,9.16,4.89,13.44,5.19,4.27,16.8,4.27v7.64H553.11Z"
            transform="translate(-214.65 -352.75)"
          />
          <path
            fill="var(--fg)"
            d="M309.78,431.82Q333,391.67,369.7,371.59q34.6-18.84,80.31-18.84,46.33,0,80.92,18.84,36.77,19.77,59.3,60.23H585a163.34,163.34,0,0,0-57.14-43.55q-35.5-16.68-77.83-16.68-42,0-77.83,16.68Q337.89,404,315.34,431.82Z"
            transform="translate(-214.65 -352.75)"
          />
        </symbol>
        <linearGradient id="#mask-gradient">
          <stop offset="0" stopColor="black" />
          <stop offset="1" stopColor="white" />
        </linearGradient>
        <mask id="mask">
          <rect
            x="0"
            y="0"
            width="200"
            height="200"
            fill="url(#mask-gradient)"
          />
        </mask>
      </defs>
      <g className="orbits" transform="translate(420, 220)">
        <g>
          <circle
            className="orbit"
            style={{
              stroke: "url(#gradient-1)",
              // @ts-ignore
              animationDelay: 0,
            }}
            r={53.4}
          />
        </g>
        <g>
          <circle
            className="orbit"
            style={{
              stroke: "url(#gradient-2)",
              animationDelay: "0.03s",
            }}
            r={103.4}
          />
          <circle
            className="gray satellite"
            style={{
              animationDelay: "0.9s",
            }}
            cx={-69.6}
            cy={-76}
            r={5.8}
          />
        </g>
        <g>
          <circle
            className="orbit"
            style={{
              stroke: "url(#gradient-3)",
              animationDelay: "0.06s",
            }}
            r={160.4}
          />
          <circle
            className="orange satellite"
            style={{
              animationDelay: "0.8s",
            }}
            cx={102.4}
            cy={-123}
            r={5.8}
          />
        </g>
        <g>
          <circle
            className="orbit"
            style={{
              stroke: "url(#gradient-4)",
              animationDelay: "0.09s",
            }}
            r={267.3}
          />
          <circle
            className="orange satellite"
            style={{
              animationDelay: "0.6s",
            }}
            cx={-243.6}
            cy={111.4}
            r={5.8}
          />
          <circle
            className="gray satellite"
            style={{
              animationDelay: "1s",
            }}
            cx={250}
            cy={94.4}
            r={5.8}
          />
          <circle
            className="orange satellite"
            style={{
              animationDelay: "0.7s",
            }}
            cx={-236.6}
            cy={-123.6}
            r={5.8}
          />
        </g>
        <g>
          <circle
            className="orbit"
            style={{
              stroke: "url(#gradient-5)",
              animationDelay: "0.12s",
            }}
            r={388.5}
          />
        </g>
      </g>
      <path
        id="vercel-logo"
        d="m336.4 261-46.2-80-46.2 80h92.4z"
        style={{
          fill: "url(#gradient-vercel)",
        }}
      />
      <g id="center">
        <path
          d="M420 202 v36 M 402 220h36"
          style={{
            strokeWidth: 3.5625,
            strokeLinecap: "round",
            strokeLinejoin: "round",
            stroke: "#999",
            fill: "none",
          }}
        />
      </g>
      <use href="#react-logo" width={120} x={500} />
      <use href="#react-logo" width={120} x={500} mask="url(#mask)" />
    </svg>
  );
}
