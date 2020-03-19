# Cluster Cost

Analyze and predict operating cost for a service in Kubernetes on AWS.

## Install

Use the below command to install binary or build from source.

Binary install:  

```bash
curl https://themindcompany.github.io/cluster-cost/install.sh -sS | bash -s
```

Source install:

```bash
git clone https://github.com/TheMindCompany/cluster-cost.git
cd cluster-cost
make build
make install
```

### Configuration

In an effort to make configuration simple we have defined a YAML config to handle node type mapping.  This config can managed through `~/.cluster_cost/config.yaml`.  To change the base path you can set it with an environment field `CLUSTER_COST_CONFIG_BASE`.

The first time you run the cli utility you will be prompted to configure node types if no config not found.  

**NOTE** Node type price is expected to be by the month.

**~/.cluster_cost/config.yaml**  
```yaml
kind: config
version: alpha/1.0
specs:
  nodeTypes:
    2x.xhighmem:
      price: 50.11
      cpu: 8
      memory: 32
```

### Autocompletion

For convenience purposes autocompletion scripts have been provided for most major shell programs.  Hopefully this make it more useable for daily use if engineers.

More information for each completion script provided:

```bash
cluster-cost configuration --help
```

## USAGE

Refer to the help menu for details `-h` or `--help`.

```bash

```

### Using the CLI

**Single pod calculation on 2x.xhighmem.**
```bash
cluster-cost predict -n 2x.xhighmem --cpu 3 --memory 9
```

**20 pod calculation on 2x.xhighmem.**
```bash
cluster-cost predict -n 2x.xhighmem --cpu 3 --memory 9 --scale 20
```

### Using as Daemon

Daemon mode runs cluster_cost as HTTP REST service.  You can either use the options arguments to predefine route values or environment value equivalents. See below examples.

---

#### Run As Daemon

```bash
cluster-cost --daemon
```

##### GET /predict/{node-type}/{cpu}/{memory}/{scale}

Response:  
```json
{
  "data": {
    "attributes": {
      "url": "<signed-url>",
      "method": "PUT",
      "ttl": 60000,
      "engine": "aws",
      "request": "<uri-received-for-request>"
    }
  }
}
```

Find more ways to configure it using `--help` flag. The above two use case examples show routes accessible when deployed with different parameters.


---

## Work In Progress
Feel free to contribute or use in any way.

Built with Rust love <3
