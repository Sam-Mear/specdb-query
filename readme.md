```
{
	specDb{
    name,
    partType{
      __typename
      ... on Cpu{
        tdp{value},
        coreCount{value},
        threadCount{value},
        baseFrequency{value}
      }
      ... on GraphicsCard {
        vramCapacity{value},
        gpuBaseFrequency{value},
        shaderProcessorCount{value}
      }
    }
  }
}
```

https://async-graphql.github.io/async-graphql/en/define_input_object.html