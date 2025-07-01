# topup-cipher-vault

---

# 🛰️ Rust + Java Microservice Architecture

This project is a hybrid microservices system combining Java Spring Boot services with a Rust gRPC server. It uses Eureka for service discovery and a Spring Cloud Gateway for routing. 

The core functionality includes generating, reserving, taking, and uploading encrypted pin codes.

The Rust service performs secure, high-performance tasks (like PIN code encryption and file chunk handling), while Java handles app configuration, service registration, and external APIs.

---
# 🧱 Architecture Overview
- **Client** interacts with the system via REST endpoints exposed by the Spring Boot services behind the API Gateway.
- **Spring Boot + Gateway + Eureka** handle service discovery, routing, and REST API orchestration.
- Communication between Java services and the **Rust gRPC server** happens over gRPC for efficient, high-performance interactions.
- The **Rust Service** handles core business logic related to pin code storage and management, interacting with **MongoDB** for persistent data storage.

```
                            [ Client ] --> [ Spring Boot (Java) + Gateway + Eureka ]
                                                     |
                                                     | gRPC
                                                     v
                                     [ Rust Service (gRPC server) ]
                                                     |
                                                     v
                                             [ MongoDB Database ]
```
---
# ⚙️ Tech Stack


| Component           | Technology                                                                                                                                                      |
|---------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `Java App`          | `Spring Boot, Spring Cloud Gateway, Eureka`                                                                                                                     | Auto-generated primary key for each configuration entry.                                     |
| `Rust Service`      | Rust, Tokio, Tonic (gRPC)                                                                                                |
| `Gateway`           | Spring Cloud Gateway                                                                                      |
| `Service Registry`  | Eureka                                                                                  |
| `Data Store`        | MongoDB (via mongodb crate in Rust)                                                                                                |
| `Communication`     | gRPC + Protocol Buffers                                                                |
| `Encryption`        | AES-GCM (via aes-gcm crate or ring) |

---


## 🧰 Build Instructions

### ☕ Java Build

```bash
# Build all Java services (assuming Maven wrapper)
./mvnw clean install
```

### 🔧 Rust Build

```bash
# install protobuf compiler
sudo apt update
sudo apt install protobuf-compiler

# Build Rust gRPC server
cd rust-pin-service
cargo build --release
```

---

## 🚀 Running the Application

### Recommended Startup Order

1. **Start MongoDB** (if not running already)
   - **DB** : pin-vault
   - **Collections** : pincodes | reserved-pins
3. **Start Eureka Registry**

```bash
java -jar dicovery/target/dicovery-0.0.1-SNAPSHOT.jar
```

3. **Start API Gateway**

```bash
java -jar gateway/target/gateway-0.0.1-SNAPSHOT.jar
```

4. **Start Core Java Services**

```bash
java -jar core-service/target/core-service-0.0.1-SNAPSHOT.jar
```

5. **Start Rust gRPC Server**

```bash
cd rust-pin-service
cargo run --release
```

---

## 🔧 Core API Endpoints (via Gateway) and Example cURL Commands

### 🧪  Generate Pin Codes (POST)

**cURL:**

```bash
curl -X POST http://localhost:8081/core/api/v1/pin-code/generate \
-H "Content-Type: application/json" \
-d '{"count":10000}'
```

---

### 🔍 Get Pin Code Status (GET)

```
GET http://localhost:8081/core/api/v1/pin-code/{id}
```

**Example cURL:**

```bash
curl http://localhost:8081/core/api/v1/pin-code/68623be237fcb79b8cf894fa
```

---

### 📦 Reserve a Pin Code (POST)

**cURL:**

```bash
curl -X POST http://localhost:8081/core/api/v1/pin-code/reserve
```

> The response returns a `reservationId` used in the next step.

---

### 🎯 Take a Pin Code (POST)

**cURL:**

```bash
curl -X POST http://localhost:8081/core/api/v1/pin-code/take \
-H "Content-Type: application/json" \
-d '{"reservationId":"68625641c7582e68902b0f16"}'
```

---

### 📤 Upload Encrypted Pin Codes (Multipart File Upload)

```
POST http://localhost:8081/core/api/v1/pin-code/upload
Content-Type: multipart/form-data
```

**Form Data:**

- `file` → The file to upload (e.g., `pin.txt` provided in project root)

**cURL:**

```bash
curl -X POST http://localhost:8081/core/api/v1/pin-code/upload \
-F "file=@pin.txt"
```

---

## ⚙️ Configuration

- Java services configured via `application.yml` or `application.properties`.
- Rust service configured via environment variables or config.yml file in the project root 

---

## 🔧 Troubleshooting

- Ensure MongoDB is running and accessible.
- Verify Eureka server is up before starting gateway and core services.
- Check logs for gRPC connectivity errors.
- Confirm service registration on discovery html entry.

---
### 📬 Contact

For questions or collaboration ideas, feel free to reach out.