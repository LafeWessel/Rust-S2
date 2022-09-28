FROM ubuntu:20.04
RUN apt-get update
RUN apt-get install -y cmake 
RUN apt-get install -y git
RUN apt-get install -y libssl-dev
RUN apt-get install -y curl
RUN apt-get install -y gcc build-essential

# install abseil
WORKDIR /abseil
RUN git clone https://github.com/abseil/abseil-cpp.git
RUN cmake -S /abseil/abseil-cpp -B /abseil/build/abseil-cpp -DCMAKE_POSITION_INDEPENDENT_CODE=ON -DCMAKE_CXX_STANDARD=17 -DCMAKE_INSTALL_PREFIX=/absl -DABSL_ENABLE_INSTALL=ON -DBUILD_TESTING=OFF -DABSL_PROPAGATE_CXX_STD=ON
RUN cmake --build /abseil/build/abseil-cpp --target install

# install S2 geometry
WORKDIR /s2
RUN git clone https://github.com/google/s2geometry.git
WORKDIR /s2/s2geometry/build
RUN cmake -DCMAKE_CXX_STANDARD=17 -DCMAKE_PREFIX_PATH=/absl -DWITH_PYTHON=OFF -DCMAKE_INSTALL_PREFIX=/s2/bin ..
RUN make install

WORKDIR /code
CMD ["/bin/bash"]