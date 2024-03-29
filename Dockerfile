FROM rust:1.65

RUN apt-get update
RUN apt-get install -y cmake git libssl-dev curl gcc build-essential
 
# install abseil
WORKDIR /abseil
RUN git clone https://github.com/abseil/abseil-cpp.git
# RUN cmake -S /abseil/abseil-cpp -B /abseil/build/abseil-cpp -DCMAKE_POSITION_INDEPENDENT_CODE=ON -DCMAKE_CXX_STANDARD=17 -DCMAKE_INSTALL_PREFIX=/absl -DABSL_ENABLE_INSTALL=ON -DBUILD_TESTING=OFF -DABSL_PROPAGATE_CXX_STD=ON
# RUN cmake --build /abseil/build/abseil-cpp --target install

# install S2 geometry
WORKDIR /s2
RUN git clone https://github.com/google/s2geometry.git
WORKDIR /s2/s2geometry/build
# RUN cmake -DCMAKE_CXX_STANDARD=17 -DCMAKE_PREFIX_PATH=/absl -DWITH_PYTHON=OFF -DCMAKE_INSTALL_PREFIX=/s2/bin ..
# RUN make install

# install Rust
# RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# install bindgen dependencies
RUN apt-get install -y llvm-dev libclang-dev clang
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /code
CMD ["/bin/bash"]