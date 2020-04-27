package cz.cvut.swa.bazaar.productservice.service

import com.fasterxml.jackson.databind.ObjectMapper
import cz.cvut.swa.bazaar.productservice.data.ReviewDTO
import org.junit.AfterClass
import org.junit.Assert
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith
import org.mockserver.integration.ClientAndServer
import org.mockserver.integration.ClientAndServer.startClientAndServer
import org.mockserver.model.Header
import org.mockserver.model.HttpRequest.request
import org.mockserver.model.HttpResponse.response
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.boot.test.context.SpringBootTest
import org.springframework.test.context.junit4.SpringRunner
import java.time.LocalDateTime
import java.util.*

@SpringBootTest
@RunWith(SpringRunner::class)
class ReviewServiceMockTest {

    @Autowired
    private lateinit var reviewService: ReviewService

    @Autowired
    private lateinit var objectMapper: ObjectMapper

    companion object {
        lateinit var mockServer: ClientAndServer

        @JvmStatic
        @BeforeClass
        fun startServer() {
            mockServer = startClientAndServer(9090)
        }

        @JvmStatic
        @AfterClass
        fun stopServer() {
            mockServer.stop()
        }
    }

    @Test
    fun fetchProductReviews_shouldCallMockServer_shouldReturnReviews() {
        // given
        val productId = randomUuid()
        val reviewList = listOf(
                ReviewDTO(randomUuid(), randomUuid(), "The product is absolutely perfect!", LocalDateTime.now()),
                ReviewDTO(randomUuid(), randomUuid(), "It's a scam!!", LocalDateTime.now())
        )

        mockServer.`when`(
                request()
                        .withMethod("GET")
                        .withPath("/reviews/$productId"))
                .respond(
                        response()
                                .withStatusCode(200)
                                .withHeaders(
                                        Header("Content-Type", "application/json; charset=utf-8"))
                                .withBody(objectMapper.writeValueAsString(reviewList))
                )

        // when
        val result = reviewService.fetchProductReviews(productId)

        // verify
        Assert.assertNotNull(result)
        Assert.assertEquals(2, result?.size)
        Assert.assertArrayEquals(reviewList.toTypedArray(), result?.toTypedArray()) // verify by deep array equals
    }

    private fun randomUuid() = UUID.randomUUID().toString()

}